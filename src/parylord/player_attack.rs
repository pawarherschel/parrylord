use crate::parylord::assets::PlayerAssets;
use crate::parylord::enemy_attack::EnemyAttack;
use crate::parylord::player::Player;
use crate::parylord::CollisionLayer;
use crate::screens::Screen;
use crate::{exponential_decay, AppSystems, PausableSystems};
use avian2d::parry::na::SimdBool;
use avian2d::prelude::{Collider, CollidingEntities, CollisionLayers, LayerMask, Sensor};
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::tasks::futures_lite::StreamExt;
use log::{log, Level};
use std::sync::{Arc, Mutex};

pub fn plugin(app: &mut App) {
    app.register_type::<PlayerAttackIndicator>();
    app.register_type::<PlayerAttack>();

    app.add_systems(
        Update,
        (aim, get_parry_attempt.pipe(handle_parries))
            .run_if(in_state(Screen::Gameplay))
            .in_set(AppSystems::RecordInput)
            .in_set(PausableSystems),
    );
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct PlayerAttackIndicator;

impl PlayerAttackIndicator {
    pub fn spawn(player_assets: &PlayerAssets) -> impl Bundle {
        (
            Name::new("PlayerAttackIndicator"),
            Self,
            Sprite {
                image: player_assets.attack_indicator.clone(),
                anchor: Anchor::CenterLeft,
                ..default()
            },
            Collider::triangle(
                Vec2::new(0.0, 0.0),
                Vec2::new(190.0, -140.0),
                Vec2::new(190.0, 140.0),
            ),
            CollisionLayers::new(
                [CollisionLayer::PlayerParry],
                [CollisionLayer::EnemyProjectile],
            ),
            Sensor,
            CollidingEntities::default(),
        )
    }
}

fn aim(
    window: Single<&Window>,
    mut attack_indicator: Query<
        (&mut Transform, &GlobalTransform),
        (With<PlayerAttackIndicator>, Without<Player>),
    >,
    camera: Single<(&Camera, &GlobalTransform)>,
    time: Res<Time>,
) -> Result {
    let window = *window;

    let Some(mouse) = window.cursor_position() else {
        return Ok(());
    };

    let (mut attack_indicator, gt) = attack_indicator.single_mut()?;
    let (camera, camera_transform) = *camera;

    let Ok(pos) = camera.viewport_to_world(camera_transform, mouse) else {
        return Ok(());
    };
    let pos = pos.origin.truncate();

    let vec_to_mouse = (pos.extend(gt.translation().z) - gt.translation()).normalize_or_zero();
    let alpha = vec_to_mouse.y.atan2(vec_to_mouse.x);

    let mut curr_quat = attack_indicator.rotation;
    let mut target_quat = Quat::from_rotation_z(alpha);

    let dot = curr_quat.dot(target_quat);
    if dot < 0.0 {
        curr_quat = -curr_quat;
        target_quat = -target_quat;
    }

    // https://docs.rs/glam/0.29.3/src/glam/f32/sse2/quat.rs.html#665
    // Note that a rotation can be represented by two quaternions: `q` and
    // `-q`. The slerp path between `q` and `end` will be different from the
    // path between `-q` and `end`. One path will take the long way around and
    // one will take the short way. In order to correct for this, the `dot`
    // product between `self` and `end` should be positive. If the `dot`
    // product is negative, slerp between `self` and `-end`.
    // let mut dot = self.dot(end);
    // if dot < 0.0 {
    //     end = -end;
    //     dot = -dot;
    // }

    let interpolated_quat = exponential_decay!(
        current: curr_quat,
        target: target_quat,
        delta: time.delta_secs(),
    )
    .normalize();

    attack_indicator.rotation = interpolated_quat;

    Ok(())
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct PlayerAttack;

pub fn get_parry_attempt(
    query: Single<&CollidingEntities, (With<PlayerAttackIndicator>, Without<Player>)>,
    all_player_projectiles: Query<Entity, With<PlayerAttack>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
) -> Option<Vec<Entity>> {
    if !mouse_input.just_pressed(MouseButton::Left) {
        return None;
    }

    Some(
        query
            .iter()
            .copied()
            .filter(|e| !all_player_projectiles.contains(*e))
            .collect(),
    )
}

pub fn handle_parries(In(entities): In<Option<Vec<Entity>>>, mut commands: Commands) {
    let Some(entities) = entities else {
        return;
    };

    for entity in entities {
        commands.entity(entity).remove::<EnemyAttack>();
        commands.entity(entity).insert(PlayerAttack);

        commands
            .entity(entity)
            .entry()
            .and_modify(|mut collision_layers: Mut<CollisionLayers>| {
                log!(Level::Info, "from: {collision_layers:?}");
                collision_layers.memberships = LayerMask::from([CollisionLayer::PlayerProjectile]);
                collision_layers.filters = LayerMask::from([CollisionLayer::Enemy]);
                log!(Level::Info, "to: {collision_layers:?}");
            });
    }
}
