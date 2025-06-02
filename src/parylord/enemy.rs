use crate::asset_tracking::LoadResource;
use crate::parylord::health::{Health, InvincibilityTimer, ZeroHealth};
use crate::parylord::CollisionLayer;
use crate::screens::Screen;
use crate::PausableSystems;
use avian2d::prelude::{Collider, CollidingEntities, CollisionLayers, RigidBody, Sensor};
use bevy::ecs::spawn::SpawnableList;
use bevy::prelude::*;
use log::{log, Level};
use rand::{thread_rng, Rng};
use std::thread::spawn;

pub fn plugin(app: &mut App) {
    app.register_type::<EnemyAssets>();

    app.register_type::<EnemyAssets>();
    app.load_resource::<EnemyAssets>();

    app.add_systems(
        Update,
        (hurt, handle_dead_enemies)
            .run_if(in_state(Screen::Gameplay))
            .in_set(PausableSystems),
    );
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Enemy;

impl Enemy {
    pub fn spawn(enemy_assets: &EnemyAssets, position: Vec2) -> impl Bundle {
        let mut rng = thread_rng();
        let pick = rng.gen_range(0..=EnemyAssets::MAX_ASSETS);
        (
            Self,
            Health(15),
            Transform::from_translation(position.extend(1.0)),
            Sprite {
                image: match pick % EnemyAssets::MAX_ASSETS {
                    0 => enemy_assets.beige.clone(),
                    1 => enemy_assets.blue.clone(),
                    2 => enemy_assets.green.clone(),
                    3 => enemy_assets.yellow.clone(),
                    _ => {
                        unreachable!()
                    }
                },
                ..default()
            },
            RigidBody::Dynamic,
            Sensor,
            Collider::circle(64.0),
            CollisionLayers::new(
                [CollisionLayer::Enemy],
                [
                    CollisionLayer::Walls,
                    CollisionLayer::Player,
                    CollisionLayer::PlayerProjectile,
                    CollisionLayer::PlayerHurt,
                ],
            ),
            CollidingEntities::default(),
        )
    }
}

pub fn hurt(
    mut q: Query<
        (&mut Health, &CollidingEntities, Entity),
        (With<Enemy>, Without<InvincibilityTimer>),
    >,
    mut commands: Commands,
) {
    for (mut health, collisions, entity) in &mut q {
        if !collisions.0.is_empty() {
            health.0 -= 1;

            commands
                .entity(entity)
                .insert(InvincibilityTimer(Timer::from_seconds(
                    0.2,
                    TimerMode::Once,
                )));
        }
    }
}

pub fn handle_dead_enemies(
    dead_enemies: Query<Entity, (With<ZeroHealth>, With<Enemy>)>,
    mut commands: Commands,
) {
    for entity in dead_enemies {
        commands.entity(entity).despawn();
    }
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct EnemyAssets {
    #[dependency]
    beige: Handle<Image>,
    #[dependency]
    blue: Handle<Image>,
    #[dependency]
    green: Handle<Image>,
    #[dependency]
    yellow: Handle<Image>,
}

impl EnemyAssets {
    const MAX_ASSETS: u8 = 4;
}

impl FromWorld for EnemyAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();

        Self {
            beige: assets.load("images/enemy/beige.png"),
            blue: assets.load("images/enemy/blue.png"),
            green: assets.load("images/enemy/green.png"),
            yellow: assets.load("images/enemy/yellow.png"),
        }
    }
}
