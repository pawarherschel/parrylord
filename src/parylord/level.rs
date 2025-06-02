use crate::asset_tracking::LoadResource;
use crate::parylord::enemy::{Enemy, EnemyAssets};
use crate::parylord::player::{Player, PlayerAssets};
use crate::parylord::CollisionLayer;
use crate::screens::Screen;
use avian2d::prelude::{Collider, CollisionLayers, RigidBody};
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<LevelAssets>();
    app.register_type::<Walls>();
    app.load_resource::<LevelAssets>();
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct LevelAssets {
    #[dependency]
    bg: Handle<Image>,
}

impl FromWorld for LevelAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            bg: assets.load("images/bg.png"),
        }
    }
}

/// A system that spawns the main level.
pub fn spawn_level(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    player_assets: Res<PlayerAssets>,
    enemy_assets: Res<EnemyAssets>,
) {
    commands.spawn((
        Name::new("Level"),
        Transform::default(),
        Visibility::default(),
        StateScoped(Screen::Gameplay),
        children![
            Player::spawn(&player_assets),
            (
                Sprite {
                    image: level_assets.bg.clone(),
                    ..default()
                },
                Transform::from_xyz(0.0, 0.0, -1000.0),
                walls(),
            ),
            Enemy::spawn(&enemy_assets, Vec2::new(150.0, 150.0)),
        ],
    ));
}

pub fn walls() -> impl Bundle {
    (
        Name::new("Walls"),
        Walls,
        children![
            (
                Name::new("Left Wall"),
                Transform::from_xyz(-(1920.0 / 2.0 - 96.0 / 2.0), 0.0, 0.0),
                RigidBody::Static,
                Collider::rectangle(96.0, 1080.0),
                CollisionLayers::new([CollisionLayer::Walls], [CollisionLayer::Player])
            ),
            (
                Name::new("Right Wall"),
                Transform::from_xyz(1920.0 / 2.0 - 96.0 / 2.0, 0.0, 0.0),
                RigidBody::Static,
                Collider::rectangle(96.0, 1080.0),
                CollisionLayers::new([CollisionLayer::Walls], [CollisionLayer::Player])
            ),
            (
                Name::new("Top Wall"),
                Transform::from_xyz(0.0, 1080.0 / 2.0 - 96.0 / 2.0, 0.0),
                RigidBody::Static,
                Collider::rectangle(1920.0, 96.0),
                CollisionLayers::new([CollisionLayer::Walls], [CollisionLayer::Player])
            ),
            (
                Name::new("Top Wall"),
                Transform::from_xyz(0.0, -(1080.0 / 2.0 - 96.0 / 2.0), 0.0),
                RigidBody::Static,
                Collider::rectangle(1920.0, 96.0),
                CollisionLayers::new([CollisionLayer::Walls], [CollisionLayer::Player])
            )
        ],
    )
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Walls;
