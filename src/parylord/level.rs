use crate::asset_tracking::LoadResource;
use crate::parylord::assets::{EnemyAssets, LevelAssets, PlayerAssets};
use crate::parylord::enemy::Enemy;
use crate::parylord::enemy_attack::EnemyAttack;
use crate::parylord::player::Player;
use crate::parylord::ttl::TTL;
use crate::parylord::CollisionLayer;
use crate::parylord::CollisionLayer::EnemyProjectile;
use crate::screens::Screen;
use avian2d::prelude::{Collider, CollisionLayers, LinearVelocity, RigidBody};
use bevy::prelude::*;
use std::thread::spawn;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<LevelAssets>();
    app.register_type::<LevelBackground>();
    app.register_type::<Walls>();
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
            (Player::spawn(&player_assets)),
            (LevelBackground::spawn(&level_assets)),
            (Enemy::spawn(&enemy_assets, Vec2::new(150.0, 150.0))),
            (EnemyAttack::spawn(
                &enemy_assets,
                Vec2::new(150.0, -150.0),
                LinearVelocity::ZERO,
                TTL::new(1000.0)
            )),
        ],
    ));
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct LevelBackground;

impl LevelBackground {
    pub fn spawn(level_assets: &LevelAssets) -> impl Bundle {
        (
            Name::new("Level Background"),
            Self,
            Sprite {
                image: level_assets.bg.clone(),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, -1000.0),
            // children![Walls::spawn()],
        )
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Walls;

impl Walls {
    pub fn spawn() -> impl Bundle {
        (
            Name::new("Walls"),
            Self,
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
}
