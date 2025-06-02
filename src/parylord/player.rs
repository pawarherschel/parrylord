use crate::asset_tracking::LoadResource;
use crate::exponential_decay;
use crate::parylord::dynamic_character_2d::CharacterControllerBundle;
use crate::parylord::CollisionLayer;
use crate::screens::Screen;
use crate::{AppSystems, PausableSystems};
use avian2d::prelude::{Collider, CollidingEntities, CollisionLayers, Sensor};
use bevy::prelude::*;
use bevy::sprite::Anchor;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Player>();
    app.register_type::<AttackIndicator>();

    app.register_type::<PlayerAssets>();
    app.load_resource::<PlayerAssets>();

    // Record directional input as movement controls.
    app.add_systems(
        Update,
        (aim, hurt)
            .run_if(in_state(Screen::Gameplay))
            .in_set(AppSystems::RecordInput)
            .in_set(PausableSystems),
    );
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct PlayerHurtBox;
impl Player {
    pub fn spawn(player_assets: &PlayerAssets) -> impl Bundle {
        // A texture atlas is a way to split a single image into a grid of related images.
        // You can learn more in this example: https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs
        // let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 6, 2, Some(UVec2::splat(1)), None);
        // let texture_atlas_layout = texture_atlas_layouts.add(layout);
        // let player_animation = PlayerAnimation::new();

        (
            Name::new("Player"),
            Self,
            CharacterControllerBundle::new(Collider::capsule(48.0, 48.0)),
            Sprite {
                image: player_assets.pink.clone(),
                // texture_atlas: Some(TextureAtlas {
                //     layout: texture_atlas_layout,
                //     index: player_animation.get_atlas_index(),
                // }),
                anchor: Anchor::Center,
                ..default()
            },
            children![
                (
                    Name::new("PlayerAttackIndicator"),
                    Sprite {
                        image: player_assets.attack_indicator.clone(),
                        anchor: Anchor::CenterLeft,
                        ..default()
                    },
                    AttackIndicator,
                ),
                (
                    Name::new("CollisionLayer::PlayerHurt"),
                    PlayerHurtBox,
                    Collider::capsule(32.0, 32.0),
                    Sensor,
                    CollisionLayers::new(
                        [CollisionLayer::PlayerHurt],
                        [CollisionLayer::Enemy, CollisionLayer::EnemyProjectile]
                    ),
                    CollidingEntities::default(),
                ),
            ],
            Transform::from_scale(Vec2::splat(0.5).extend(1.0)),
        )
    }
}

fn hurt(query: Query<&CollidingEntities, With<PlayerHurtBox>>) {
    for q in query {
        if !q.is_empty() {
            // log!(Level::Info, "{q:?}");
        }
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
struct AttackIndicator;

fn aim(
    window: Single<&Window>,
    mut attack_indicator: Query<
        (&mut Transform, &GlobalTransform),
        (With<AttackIndicator>, Without<Player>),
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

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct PlayerAssets {
    #[dependency]
    pink: Handle<Image>,
    #[dependency]
    attack_indicator: Handle<Image>,
}

impl FromWorld for PlayerAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            pink: assets.load("images/pink/front.png"),

            attack_indicator: assets.load("images/pink/attack_indicator.png"),
        }
    }
}
