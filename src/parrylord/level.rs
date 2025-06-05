use crate::parrylord::assets::{LevelAssets, PlayerAssets};
use crate::parrylord::enemy::{Enemy, SpawnEnemy};
use crate::parrylord::player::Player;
use crate::parrylord::CollisionLayer;
use crate::screens::Screen;
use crate::{ParrylordSingleton, PausableSystems};
use avian2d::prelude::{Collider, CollisionLayers, RigidBody};
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.register_type::<Level>();
    app.register_type::<EnemySpawn>();
    app.register_type::<LevelAssets>();
    app.register_type::<LevelBackground>();
    app.register_type::<Walls>();
    app.register_type::<Wall>();

    app.add_systems(
        Update,
        new_level
            .run_if(in_state(Screen::Gameplay))
            .in_set(PausableSystems),
    );
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Level;

impl Level {
    pub fn bundle(level_assets: &LevelAssets, player_assets: &PlayerAssets) -> impl Bundle {
        (
            StateScoped(Screen::Gameplay),
            Name::new("Level"),
            Self,
            Transform::default(),
            Visibility::default(),
            children![
                Player::bundle(player_assets),
                LevelBackground::bundle(level_assets),
                EnemySpawn,
            ],
        )
    }
}

/// A system that spawns the main level.
pub fn spawn_level(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    player_assets: Res<PlayerAssets>,
    mut parrylord_singleton: ResMut<ParrylordSingleton>,
) {
    commands.spawn(Level::bundle(&level_assets, &player_assets));
    *parrylord_singleton = ParrylordSingleton::default();
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct EnemySpawn;

fn new_level(
    enemies: Query<(), With<Enemy>>,
    mut spawn_enemy_event_writer: EventWriter<SpawnEnemy>,
    mut singleton: ResMut<ParrylordSingleton>,
) {
    if !enemies.is_empty() {
        return;
    }

    for _ in 0..singleton.level {
        spawn_enemy_event_writer.write(SpawnEnemy);
    }

    singleton.level += 1;
}

//     let root = context.entity;
//     let level = world.resource::<ParrylordLevel>().0;
//     let enemy_assets = world.resource::<EnemyAssets>().clone();
//
//     let mut commands = world.commands();
//
//     let mut entities = Vec::with_capacity(level as usize);
//
//     for _ in 0..level {
//         entities.push(
//             commands
//                 .spawn(Enemy::bundle(&enemy_assets, Vec2::new(150.0, -150.0)))
//                 .id(),
//         );
//     }
//
//     commands.entity(root).add_children(&entities);
// }

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct LevelBackground;

impl LevelBackground {
    pub fn bundle(level_assets: &LevelAssets) -> impl Bundle {
        (
            // StateScoped(Screen::Gameplay),
            // Name::new("Level Background"),
            Self,
            Sprite {
                image: level_assets.bg.clone(),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, -1000.0),
            Walls::bundle(),
        )
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Walls;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Wall;

impl Walls {
    pub fn bundle() -> impl Bundle {
        (
            Name::new("Walls"),
            Self,
            children![
                (
                    Name::new("Left Wall"),
                    Wall,
                    Transform::from_xyz(-(1920.0 / 2.0 - 96.0 / 2.0), 0.0, 0.0),
                    RigidBody::Static,
                    Collider::rectangle(96.0, 1080.0),
                    CollisionLayers::new(
                        [CollisionLayer::Walls],
                        [
                            CollisionLayer::Player,
                            CollisionLayer::PlayerProjectile,
                            CollisionLayer::Enemy,
                            CollisionLayer::EnemyProjectile
                        ]
                    )
                ),
                (
                    Name::new("Right Wall"),
                    Wall,
                    Transform::from_xyz(1920.0 / 2.0 - 96.0 / 2.0, 0.0, 0.0),
                    RigidBody::Static,
                    Collider::rectangle(96.0, 1080.0),
                    CollisionLayers::new(
                        [CollisionLayer::Walls],
                        [
                            CollisionLayer::Player,
                            CollisionLayer::PlayerProjectile,
                            CollisionLayer::Enemy,
                            CollisionLayer::EnemyProjectile
                        ]
                    )
                ),
                (
                    Name::new("Top Wall"),
                    Wall,
                    Transform::from_xyz(0.0, 1080.0 / 2.0 - 96.0 / 2.0, 0.0),
                    RigidBody::Static,
                    Collider::rectangle(1920.0, 96.0),
                    CollisionLayers::new(
                        [CollisionLayer::Walls],
                        [
                            CollisionLayer::Player,
                            CollisionLayer::PlayerProjectile,
                            CollisionLayer::Enemy,
                            CollisionLayer::EnemyProjectile
                        ]
                    )
                ),
                (
                    Name::new("Top Wall"),
                    Wall,
                    Transform::from_xyz(0.0, -(1080.0 / 2.0 - 96.0 / 2.0), 0.0),
                    RigidBody::Static,
                    Collider::rectangle(1920.0, 96.0),
                    CollisionLayers::new(
                        [CollisionLayer::Walls],
                        [
                            CollisionLayer::Player,
                            CollisionLayer::PlayerProjectile,
                            CollisionLayer::Enemy,
                            CollisionLayer::EnemyProjectile
                        ]
                    )
                )
            ],
        )
    }
}
