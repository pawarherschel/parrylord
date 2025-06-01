use crate::asset_tracking::LoadResource;
use crate::parylord::movement::{MovementController, ScreenWrap};
use crate::screens::Screen;
use crate::{AppSystems, PausableSystems};
use bevy::prelude::ops::asin;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::window::WindowResolution;
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
struct Player;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
struct AttackIndicator;

/// The player character.
pub fn player(
    max_speed: f32,
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
        MovementController {
            max_speed,
            ..default()
        },
        ScreenWrap,
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
    window: Query<&Window>,
    mut attack_indicator: Query<
        (&mut Transform, &GlobalTransform),
        (With<AttackIndicator>, Without<Player>),
    >,
    camera: Single<(&Camera, &GlobalTransform)>,
) -> Result {
    let Ok(window) = window.single() else {
        return Ok(());
    };

    let Some(mouse) = window.cursor_position() else {
        return Ok(());
    };

    let (mut attack_indicator, gt) = attack_indicator.single_mut()?;
    let (camera, camera_transform) = *camera;

    let Ok(pos) = camera.viewport_to_world(camera_transform, mouse) else {
        return Ok(());
    };
    let pos = pos.origin.truncate();

    let vec_to_mouse = (gt.translation() - pos.extend(gt.translation().z)).normalize_or_zero();
    let alpha = asin(vec_to_mouse.y);
    let alpha = if pos.x < gt.translation().x {
        alpha
    } else {
        -alpha
    };

    attack_indicator.rotation = Quat::from_axis_angle(attack_indicator.local_z().as_vec3(), alpha);

    if pos.x < gt.translation().x {
        attack_indicator.scale.x = -1.0;
        attack_indicator.scale.y = -1.0;
        attack_indicator.scale.z = -1.0;
    } else {
        attack_indicator.scale.x = 1.0;
        attack_indicator.scale.y = 1.0;
        attack_indicator.scale.z = 1.0;
    }

    Ok(())
}

fn exponential_decay(a: Vec3, b: Vec3, decay: f32, delta: f32) -> Vec3 {
    b + (a - b) * f32::exp(-decay * delta)
}

fn window_to_game(coord: Vec3, window_resolution: WindowResolution) -> Vec3 {
    let x = window_resolution.width();
    let y = window_resolution.height();

    let xy = Vec2::new(x, y);

    let half_xy = xy / 2.0;

    let half_xy = half_xy.extend(coord.z);
    //
    // if debug {
    //     log!(Level::Info, "half_xy: {half_xy:?}");
    // }

    let a = coord - half_xy;

    // if debug {
    //     log!(Level::Info, "coord - half_xy: {:?}", coord - half_xy);
    // }

    Vec3::new(a.x, a.y, a.z)
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
