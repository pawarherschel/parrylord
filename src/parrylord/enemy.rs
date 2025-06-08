use crate::assets::{AttackAssets, EnemyAssets};
use crate::audio::sound_effect;
use crate::parrylord::enemy_attack::EnemyAttack;
use crate::parrylord::health::{DisplayHealth, Health, ZeroHealth};
use crate::parrylord::player::Player;
use crate::parrylord::ttl::Ttl;
use crate::parrylord::CollisionLayer;
use crate::screens::Screen;
use crate::{AppSystems, ParrylordSingleton, PausableSystems};
use avian2d::prelude::{AngularVelocity, Collider, CollisionLayers, LinearVelocity, RigidBody};
use bevy::prelude::*;
use rand::prelude::SliceRandom;
use rand::{random, Rng};

pub fn plugin(app: &mut App) {
    app.register_type::<Enemy>();
    app.register_type::<EnemyAssets>();
    app.add_event::<EnemyIntent>();
    app.add_event::<SpawnEnemy>();
    app.register_type::<EnemyStateTimer>();

    app.add_systems(
        Update,
        (handle_dead_enemies, write_enemy_intents)
            .run_if(in_state(Screen::Gameplay))
            .in_set(PausableSystems),
    );

    app.add_systems(
        PreUpdate,
        (handle_enemy_intents, handle_spawn_enemy_events)
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

#[derive(Event, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
pub struct SpawnEnemy;

pub fn handle_spawn_enemy_events(
    mut events: EventReader<SpawnEnemy>,
    mut commands: Commands,
    enemy_assets: Res<EnemyAssets>,
    singleton: Res<ParrylordSingleton>,
) {
    for _ in events.read() {
        commands.spawn(Enemy::bundle(
            &enemy_assets,
            get_random_vec2_in_play_area(),
            Enemy::BASE_HEALTH.saturating_pow(singleton.level - 1),
        ));
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Default, Reflect)]
#[reflect(Component)]
pub enum EnemyState {
    Start,
    MovingTo(Vec2),
    Attacking(u8),
    #[default]
    Idling,
}

#[derive(Reflect, Debug, Clone, Copy, PartialEq, Default, Event)]
pub enum EnemyIntent {
    #[default]
    None,
    Idle(Entity),
    Move(Entity, Vec2),
    Attack(Entity, Vec2, u8),
    GoToStart(Entity),
}

impl EnemyIntent {
    #[tracing::instrument()]
    fn get_entity(&self) -> Option<Entity> {
        match self {
            Self::None => None,
            Self::Idle(it) | Self::Move(it, _) | Self::Attack(it, _, _) | Self::GoToStart(it) => {
                Some(*it)
            }
        }
    }
}

#[tracing::instrument(skip_all)]
pub fn write_enemy_intents(
    mut enemies: Query<(
        &GlobalTransform,
        &mut Transform,
        &mut LinearVelocity,
        &mut AngularVelocity,
        &mut EnemyStateTimer,
        &mut Enemy,
        Entity,
    )>,
    player: Single<&GlobalTransform, With<Player>>,
    mut intent_writer: EventWriter<EnemyIntent>,
    singleton: Res<ParrylordSingleton>,
) {
    for (global_transform, mut transform, mut velocity, mut spin, timer, enemy, entity) in
        &mut enemies
    {
        let Enemy(state) = *enemy;
        let player_pos = player.translation().truncate();
        let timer_expired = timer.0.just_finished();
        let mut thread_rng = rand::thread_rng();
        let offset = (random::<Vec2>() * 2.0 - Vec2::splat(1.0)) * 30.0;
        let no_of_attacks =
            u8::try_from(thread_rng.gen_range(1..=(4 + singleton.level))).unwrap_or(u8::MAX);

        let id = match state {
            EnemyState::Start => {
                transform.rotation = Quat::IDENTITY;
                *velocity = LinearVelocity::ZERO;
                *spin = AngularVelocity::ZERO;

                let travel = thread_rng.gen_bool(0.5);
                if travel {
                    let to_player = thread_rng.gen_bool(0.5);
                    let pos = if to_player {
                        player_pos + offset
                    } else {
                        get_random_vec2_in_play_area() + offset
                    };

                    intent_writer.write(EnemyIntent::Move(entity, pos))
                } else {
                    let to_player = thread_rng.gen_bool(0.9);
                    let pos = if to_player {
                        player_pos + offset
                    } else {
                        get_random_vec2_in_play_area() + offset
                    };

                    intent_writer.write(EnemyIntent::Attack(entity, pos, no_of_attacks))
                }
            }
            EnemyState::MovingTo(pos) => {
                let my_position = global_transform.translation().truncate();
                let reached_destination = pos.distance_squared(my_position) < 500.0;

                if timer_expired || reached_destination {
                    let to_player = thread_rng.gen_bool(0.9);

                    let pos = if to_player {
                        player_pos + offset
                    } else {
                        get_random_vec2_in_play_area() + offset
                    };

                    intent_writer.write(EnemyIntent::Attack(entity, pos, no_of_attacks))
                } else {
                    intent_writer.write(EnemyIntent::None)
                }
            }
            EnemyState::Attacking(n) => {
                if n != 0 {
                    intent_writer.write(EnemyIntent::Attack(entity, player_pos, n - 1))
                } else {
                    intent_writer.write(EnemyIntent::Idle(entity))
                }
            }
            EnemyState::Idling => {
                transform.rotation = Quat::IDENTITY;
                *velocity = LinearVelocity::ZERO;
                *spin = AngularVelocity::ZERO;

                if timer_expired {
                    intent_writer.write(EnemyIntent::GoToStart(entity))
                } else {
                    intent_writer.write(EnemyIntent::None)
                }
            }
        };

        _ = id;
    }
}

#[tracing::instrument(skip_all)]
pub fn handle_enemy_intents(
    mut intents: EventReader<EnemyIntent>,
    mut enemies: Query<(
        &mut Enemy,
        (&GlobalTransform, &mut LinearVelocity, &mut EnemyStateTimer),
    )>,
    mut commands: Commands,
    attack_assets: Res<AttackAssets>,
) -> Result {
    if intents.is_empty() {
        return Ok(());
    }

    for &intent in intents.read() {
        if intent == EnemyIntent::None {
            continue;
        }

        let Some(enemy) = intent.get_entity() else {
            warn!("Some(enemy) = intent.get_entity()");
            continue;
        };

        let Ok((mut enemy_state, (global_transform, mut velocity, _timer))) =
            enemies.get_mut(enemy)
        else {
            warn!(
                "Ok((mut enemy_state, (global_transform, mut velocity, mut timer))) = enemies.get_mut(enemy): {:?}",
                enemies.get_mut(enemy)
            );
            continue;
        };
        enemy_state.0 = match intent {
            EnemyIntent::None => EnemyState::Start,
            EnemyIntent::Idle(_) => EnemyState::Idling,
            EnemyIntent::Move(_, pos) => {
                let curr = global_transform.translation();
                let target = pos.extend(curr.z);

                let dir = (target - curr).normalize();
                let target = dir * Enemy::SPEED;

                *velocity = LinearVelocity::from(target.truncate());

                EnemyState::MovingTo(pos)
            }
            EnemyIntent::Attack(_, pos, n) => {
                let my_pos = global_transform.translation().truncate();
                let velocity = LinearVelocity(
                    (pos - my_pos).normalize() * 500.0
                        + (random::<f32>().mul_add(2.0, -1.0) * 150.0),
                );
                commands.spawn(EnemyAttack::bundle(
                    &attack_assets,
                    my_pos,
                    velocity,
                    Ttl::new(random::<f32>().mul_add(3.0, 0.25)),
                ));

                commands.spawn(sound_effect(
                    attack_assets
                        .attack_sfx
                        .choose(&mut rand::thread_rng())
                        .expect("should exist")
                        .clone(),
                ));

                if n > 0 {
                    EnemyState::Attacking(n - 1)
                } else {
                    EnemyState::Idling
                }
            }
            EnemyIntent::GoToStart(_) => EnemyState::Start,
        }
    }

    Ok(())
}

#[derive(Component, Debug, Clone, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct EnemyStateTimer(Timer);

#[tracing::instrument(skip_all)]
pub fn tick_enemy_state_timer(mut timers: Query<&mut EnemyStateTimer>, time: Res<Time>) {
    for mut timer in &mut timers {
        timer.0.tick(time.delta());
    }
}

impl Enemy {
    const SPEED: f32 = 300.0;
    const BASE_HEALTH: u32 = 2;

    #[tracing::instrument()]
    pub fn bundle(enemy_assets: &EnemyAssets, position: Vec2, health: u32) -> impl Bundle {
        let mut rng = rand::thread_rng();
        let pick = rng.gen_range(0..=EnemyAssets::MAX_ASSETS);
        (
            StateScoped(Screen::Gameplay),
            Self::default(),
            Health(health),
            DisplayHealth::bundle(),
            EnemyStateTimer(Timer::from_seconds(2.0, TimerMode::Repeating)),
            Transform::from_translation(position.extend(1.0)).with_scale(Vec3::splat(0.8)),
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
            Collider::circle(64.0),
            CollisionLayers::new(
                [CollisionLayer::Enemy],
                [
                    CollisionLayer::Enemy,
                    CollisionLayer::Walls,
                    CollisionLayer::PlayerProjectile,
                    CollisionLayer::PlayerHurt,
                ],
            ),
        )
    }
}

#[tracing::instrument(skip_all)]
pub fn handle_dead_enemies(
    dead_enemies: Query<Entity, (With<ZeroHealth>, With<Enemy>)>,
    mut commands: Commands,
    mut singleton: ResMut<ParrylordSingleton>,
) {
    for entity in dead_enemies {
        let Ok(mut entity) = commands.get_entity(entity) else {
            continue;
        };
        entity.try_despawn();

        singleton.enemies_killed += 1;

        // info!(?singleton);
    }
}

pub fn get_random_vec2_in_play_area() -> Vec2 {
    const PLAY_AREA_X: f32 = 600.0;
    const PLAY_AREA_Y: f32 = 200.0;

    let mut thread_rng = rand::thread_rng();
    let x_extents = -PLAY_AREA_X..PLAY_AREA_X;
    let y_extents = -PLAY_AREA_Y..PLAY_AREA_Y;

    let x = thread_rng.gen_range(x_extents);
    let y = thread_rng.gen_range(y_extents);

    Vec2::new(x, y)
}
