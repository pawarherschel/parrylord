use crate::asset_tracking::LoadResource;
use crate::exponential_decay;
use crate::parylord::dynamic_character_2d::CharacterControllerBundle;
use crate::parylord::movement::MovementController;
use crate::screens::Screen;
use crate::{AppSystems, PausableSystems};
use avian2d::prelude::Collider;
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
        (record_player_directional_input, aim)
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
struct AttackIndicator;

/// The player character.
pub fn player(
    player_assets: &PlayerAssets,
    // texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> impl Bundle {
    // A texture atlas is a way to split a single image into a grid of related images.
    // You can learn more in this example: https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs
    // let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 6, 2, Some(UVec2::splat(1)), None);
    // let texture_atlas_layout = texture_atlas_layouts.add(layout);
    // let player_animation = PlayerAnimation::new();

    (
        Name::new("Player"),
        Player,
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
        children![(
            Name::new("PlayerAttackIndicator"),
            Sprite {
                image: player_assets.attack_indicator.clone(),
                anchor: Anchor::CenterLeft,
                ..default()
            },
            AttackIndicator,
        )],
        Transform::from_scale(Vec2::splat(0.5).extend(1.0)),
        // MovementController {
        //     max_speed,
        //     ..default()
        // },
        // ScreenWrap,
    )
}
fn record_player_directional_input(
    input: Res<ButtonInput<KeyCode>>,
    mut controller_query: Query<&mut MovementController, With<Player>>,
) {
    // Collect directional input.
    let mut intent = Vec2::ZERO;
    if input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::ArrowUp) {
        intent.y += 1.0;
    }
    if input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown) {
        intent.y -= 1.0;
    }
    if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
        intent.x -= 1.0;
    }
    if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
        intent.x += 1.0;
    }

    // Normalize intent so that diagonal movement is the same speed as horizontal / vertical.
    // This should be omitted if the input comes from an analog stick instead.
    let intent = intent.normalize_or_zero();

    // Apply movement intent to controllers.
    for mut controller in &mut controller_query {
        controller.intent = intent;
    }
}

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
