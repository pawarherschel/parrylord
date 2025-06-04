use crate::parylord::assets::{LevelAssets, PlayerAssets};
use crate::parylord::enemy::{Enemy, SpawnEnemy};
use crate::parylord::player::Player;
use crate::parylord::{CollisionLayer, ParrylordLevel};
use crate::screens::Screen;
use crate::PausableSystems;
use avian2d::prelude::{Collider, CollisionLayers, RigidBody};
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
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
            Name::new("Level"),
            Level,
            Transform::default(),
            Visibility::default(),
            StateScoped(Screen::Gameplay),
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
) {
    commands.spawn(Level::bundle(&level_assets, &player_assets));
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct EnemySpawn;

fn new_level(
    enemies: Query<(), With<Enemy>>,
    mut spawn_enemy_event_writer: EventWriter<SpawnEnemy>,
    mut level: ResMut<ParrylordLevel>,
) {
    if !enemies.is_empty() {
        return;
    }

    for _ in 0..level.0 {
        spawn_enemy_event_writer.write(SpawnEnemy);
    }

    level.0 += 1;
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
                        [CollisionLayer::Player, CollisionLayer::Enemy]
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
                        [CollisionLayer::Player, CollisionLayer::Enemy]
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
                        [CollisionLayer::Player, CollisionLayer::Enemy]
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
                        [CollisionLayer::Player, CollisionLayer::Enemy]
                    )
                )
            ],
        )
    }
}
