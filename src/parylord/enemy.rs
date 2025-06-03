use crate::parylord::assets::EnemyAssets;
use crate::parylord::enemy_attack::EnemyAttack;
use crate::parylord::health::{DisplayHealth, Health, InvincibilityTimer, ZeroHealth};
use crate::parylord::level::Wall;
use crate::parylord::player::Player;
use crate::parylord::ttl::Ttl;
use crate::parylord::CollisionLayer;
use crate::screens::Screen;
use crate::{AppSystems, PausableSystems};
use avian2d::prelude::{
    Collider, CollidingEntities, CollisionEventsEnabled, CollisionLayers, LinearVelocity, RigidBody,
};
use bevy::prelude::*;
use log::{log, Level};
use rand::{thread_rng, Rng};
use std::time::Duration;

pub fn plugin(app: &mut App) {
    app.register_type::<Enemy>();
    app.register_type::<EnemyAssets>();
    app.add_event::<EnemyIntent>();
    app.register_type::<EnemyStateTimer>();

    app.add_systems(
        Update,
        (hurt, handle_dead_enemies, write_enemy_intents)
            .run_if(in_state(Screen::Gameplay))
            .in_set(PausableSystems),
    );

    app.add_systems(
        PreUpdate,
        (handle_enemy_intents,)
            .run_if(in_state(Screen::Gameplay))
            .in_set(PausableSystems),
    );

    app.add_systems(
        Update,
        tick_enemy_state_timer
            .in_set(AppSystems::TickTimers)
            .run_if(in_state(Screen::Gameplay))
            .in_set(PausableSystems),
    );
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Default, Reflect)]
#[reflect(Component)]
pub struct Enemy(EnemyState);

#[derive(Component, Debug, Clone, Copy, PartialEq, Default, Reflect)]
#[reflect(Component)]
pub enum EnemyState {
    #[default]
    Start,
    MovingTo(Vec2),
    Attacking,
    Idling,
}

#[derive(Reflect, Debug, Clone, Copy, PartialEq, Default, Event)]
pub enum EnemyIntent {
    #[default]
    None,
    Idle(Entity),
    Move(Entity, Vec2),
    Attack(Entity, Vec2),
    SwitchTo(Entity, EnemyState),
}

impl EnemyIntent {
    const fn get_entity(&self) -> Option<Entity> {
        match self {
            Self::None => None,
            Self::Idle(it) | Self::Move(it, _) | Self::Attack(it, _) | Self::SwitchTo(it, _) => {
                Some(*it)
            }
        }
    }
}

pub fn write_enemy_intents(
    mut enemies: Query<(
        &GlobalTransform,
        &mut LinearVelocity,
        &mut EnemyStateTimer,
        &mut Enemy,
        Entity,
    )>,
    player: Single<&GlobalTransform, With<Player>>,
    mut intent_writer: EventWriter<EnemyIntent>,
) {
    for (global_transform, mut velocity, timer, enemy, entity) in &mut enemies {
        let Enemy(state) = *enemy;
        log!(Level::Info, "EnemyState: {state:?} Timer: {timer:?}");
        let id = match state {
            EnemyState::Start => {
                *velocity = LinearVelocity::ZERO;

                let player_pos = player.translation().truncate();
                // let my_position = global_transform.translation().truncate();

                let mut thread_rng = thread_rng();
                let attack = thread_rng.gen_bool(0.9);
                let search = thread_rng.gen_bool(0.3);
                let move_randomly = thread_rng.gen_bool(0.3);

                if search {
                    intent_writer.write(EnemyIntent::Move(entity, player_pos))
                } else if attack {
                    intent_writer.write(EnemyIntent::SwitchTo(entity, EnemyState::Attacking))
                } else if move_randomly {
                    let lower_x = -1920f32 / 2.0 + 96f32 + 200.0;
                    let higher_x = 1920f32 / 2.0 - 96f32 - 200.0;
                    let lower_y = -1080f32 / 2.0 + 96f32 + 300.0;
                    let higher_y = 1080f32 / 2.0 - 96f32 - 300.0;

                    let x_extents = lower_x..higher_x;
                    let y_extents = lower_y..higher_y;

                    let x = thread_rng.gen_range(x_extents);
                    let y = thread_rng.gen_range(y_extents);

                    let pos = Vec2::new(x, y);

                    intent_writer.write(EnemyIntent::Move(entity, pos))
                } else {
                    intent_writer.write(EnemyIntent::Idle(entity))
                }
            }
            EnemyState::MovingTo(pos) => {
                let my_position = global_transform.translation().truncate();
                log!(
                    Level::Info,
                    "pos: {pos} my_position: {my_position} pos.distance_squared(my_position): {}",
                    pos.distance_squared(my_position)
                );

                if pos.distance_squared(my_position) < 500.0 {
                    intent_writer.write(EnemyIntent::Idle(entity))
                } else {
                    intent_writer.write(EnemyIntent::Move(entity, pos))
                }
            }
            EnemyState::Attacking => {
                let player_pos = player.translation().truncate();
                let mut thread_rng = thread_rng();
                let attack: bool = thread_rng.gen_bool(0.5);
                if attack {
                    intent_writer.write(EnemyIntent::Attack(entity, player_pos))
                } else {
                    intent_writer.write(EnemyIntent::SwitchTo(entity, EnemyState::Idling))
                }
            }
            EnemyState::Idling => {
                if timer.0.finished() {
                    intent_writer.write(EnemyIntent::SwitchTo(entity, EnemyState::Start))
                } else {
                    intent_writer.write(EnemyIntent::None)
                }
            }
        };

        _ = id;
    }
}

pub fn handle_enemy_intents(
    mut intents: EventReader<EnemyIntent>,
    mut enemies: Query<(
        &mut Enemy,
        (&GlobalTransform, &mut LinearVelocity, &mut EnemyStateTimer),
    )>,
    mut commands: Commands,
    // player: Single<&GlobalTransform, With<Player>>,
    enemy_assets: Res<EnemyAssets>,
    // time: Res<Time>,
) -> Result {
    if intents.is_empty() {
        log!(Level::Info, "intents.is_empty()");
        return Ok(());
    }

    for &intent in intents.read() {
        if intent == EnemyIntent::None {
            continue;
        }

        let (mut enemy_state, (global_transform, mut velocity, mut timer)) =
            enemies.get_mut(intent.get_entity().unwrap())?;

        log!(Level::Info, "intent: {intent:?}");
        match intent {
            EnemyIntent::None => {
                enemy_state.0 = EnemyState::Start;
            }
            EnemyIntent::Idle(_) => {
                *velocity = LinearVelocity::ZERO;
                timer.0.set_duration(Duration::from_secs_f32(0.5));
                enemy_state.0 = EnemyState::Idling;
            }
            EnemyIntent::Move(_, pos) => {
                let curr = global_transform.translation();
                let target = pos.extend(curr.z);

                let dir = (target - curr).normalize();
                let target = dir * Enemy::SPEED;

                enemy_state.0 = EnemyState::MovingTo(pos);

                *velocity = LinearVelocity::from(target.truncate());
            }
            EnemyIntent::Attack(_, player_pos) => {
                let my_pos = global_transform.translation().truncate();
                let velocity = LinearVelocity((player_pos - my_pos) * 1.0);
                commands.spawn(EnemyAttack::spawn(
                    &enemy_assets,
                    my_pos,
                    velocity,
                    Ttl::new(3.0),
                ));
            }
            EnemyIntent::SwitchTo(_, state) => {
                enemy_state.0 = state;
            }
        }
    }

    Ok(())
}

#[derive(Component, Debug, Clone, PartialEq, Default, Reflect)]
#[reflect(Component)]
pub struct EnemyStateTimer(Timer);

pub fn tick_enemy_state_timer(mut timers: Query<&mut EnemyStateTimer>, time: Res<Time>) {
    for mut timer in &mut timers {
        timer.0.tick(time.delta());
    }
}

impl Enemy {
    const SPEED: f32 = 100.0;

    pub fn spawn(enemy_assets: &EnemyAssets, position: Vec2) -> impl Bundle {
        let mut rng = thread_rng();
        let pick = rng.gen_range(0..=EnemyAssets::MAX_ASSETS);
        (
            Self::default(),
            Health(15),
            DisplayHealth::spawn(),
            EnemyStateTimer(Timer::from_seconds(2.0, TimerMode::Once)),
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
            LinearVelocity::default(),
            CollisionEventsEnabled,
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
        )
    }
}

pub fn hurt(
    mut q: Query<
        (&mut Health, &CollidingEntities, Entity),
        (With<Enemy>, Without<InvincibilityTimer>),
    >,
    walls: Query<Entity, With<Wall>>,
    mut commands: Commands,
) {
    'outer: for (mut health, collisions, entity) in &mut q {
        if collisions.0.is_empty() {
            continue;
        }
        for colliding_entity in collisions.iter() {
            if walls.contains(*colliding_entity) {
                continue 'outer;
            }
        }
        health.0 -= 1;

        commands
            .entity(entity)
            .insert(InvincibilityTimer(Timer::from_seconds(
                0.2,
                TimerMode::Once,
            )));
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
