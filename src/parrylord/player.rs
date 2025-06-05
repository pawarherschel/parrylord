use crate::parrylord::CollisionLayer;
use crate::parrylord::assets::PlayerAssets;
use crate::parrylord::dynamic_character_2d::CharacterControllerBundle;
use crate::parrylord::health::{DisplayHealth, Health, InvincibilityTimer, ZeroHealth};
use crate::parrylord::player_attack::PlayerAttackIndicator;
use crate::screens::Screen;
use crate::{AppSystems, PausableSystems};
use avian2d::parry::na::RealField;
use avian2d::prelude::{Collider, CollidingEntities, CollisionLayers, LinearVelocity, Sensor};
use bevy::prelude::*;
use bevy::sprite::Anchor;
use log::{Level, log};
use std::fmt::Debug;

pub fn plugin(app: &mut App) {
    app.register_type::<Player>();
    app.register_type::<PlayerHurtBox>();
    app.register_type::<PlayerSprite>();
    app.register_type::<AnimationTimer>();

    app.add_systems(
        Update,
        (hurt, handle_player_death, walk_animation, animate_sprite)
            .run_if(in_state(Screen::Gameplay))
            .in_set(PausableSystems),
    );

    app.add_systems(
        Update,
        tick_animation_timer
            .in_set(AppSystems::TickTimers)
            .run_if(in_state(Screen::Gameplay))
            .in_set(PausableSystems),
    );
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct PlayerHurtBox;

#[derive(Component, Debug, Clone, Copy, PartialEq, Reflect)]
#[reflect(Component)]
pub enum PlayerSprite {
    Front(u8),
    Walk(f32, u8),
    Stand(f32),
}

impl Default for PlayerSprite {
    fn default() -> Self {
        Self::Front(0)
    }
}

impl PlayerSprite {
    #[must_use]
    #[tracing::instrument()]
    fn advance_frame(self) -> Self {
        match self {
            Self::Front(n) => Self::Front((n + 1) % 3),
            Self::Walk(d, n) => Self::Walk(d, (n + 1) % 2),
            Self::Stand(_) => self,
        }
    }

    #[tracing::instrument()]
    fn flip_x(&self) -> bool {
        match self {
            Self::Front(_) => false,
            Self::Walk(dir, _) | Self::Stand(dir) => !dir.is_sign_positive(),
        }
    }

    #[tracing::instrument()]
    fn get_sprite(&self, player_assets: &PlayerAssets) -> Handle<Image> {
        let PlayerAssets {
            front1,
            front2,
            walk1,
            walk2,
            stand,
            attack_indicator: _,
        } = player_assets.clone();

        match self {
            Self::Front(0) => front1,
            Self::Front(1) => front2,
            Self::Walk(_, 0) => walk1,
            Self::Walk(_, 1) => walk2,
            Self::Stand(_) => stand,

            _what => unreachable!("get_sprite: {_what:?}"),
        }
    }

    #[tracing::instrument()]
    fn max_frames(&self) -> u8 {
        match self {
            PlayerSprite::Front(_) => 2,
            PlayerSprite::Walk(_, _) => 2,
            PlayerSprite::Stand(_) => 1,
        }
    }
}

#[tracing::instrument(skip_all)]
pub fn walk_animation(player: Single<(&mut PlayerSprite, &LinearVelocity), With<Player>>) {
    let (mut player_sprite, velocity) = player.into_inner();

    const MINIMUM_SPEED: f32 = 1.0;
    const MINIMUM_X_SPEED: f32 = 20.0;

    let frame_number = match *player_sprite {
        PlayerSprite::Front(n) | PlayerSprite::Walk(_, n) => n,
        PlayerSprite::Stand(_) => 0,
    };

    let front = velocity.length_squared() < MINIMUM_SPEED * MINIMUM_SPEED;
    if front {
        *player_sprite = PlayerSprite::Front(frame_number % PlayerSprite::Front(0).max_frames());
    } else {
        let run = velocity.length_squared() > MINIMUM_X_SPEED * MINIMUM_X_SPEED;
        let facing_right = velocity.x.is_sign_positive();

        *player_sprite = match (run, facing_right) {
            (false, false) => PlayerSprite::Stand(-1.0),
            (false, true) => PlayerSprite::Stand(1.0),
            (true, false) => {
                PlayerSprite::Walk(-1.0, frame_number % PlayerSprite::Walk(0.0, 0).max_frames())
            }
            (true, true) => {
                PlayerSprite::Walk(1.0, frame_number % PlayerSprite::Walk(0.0, 0).max_frames())
            }
        }
    }
}

#[tracing::instrument(skip_all)]
fn animate_sprite(
    player: Single<(&mut Sprite, &AnimationTimer, &mut PlayerSprite), With<Player>>,
    player_assets: Res<PlayerAssets>,
) {
    let (mut sprite, timer, mut sprite_state) = player.into_inner();

    let player_assets = &*player_assets;

    sprite.flip_x = sprite_state.flip_x();
    sprite.image = sprite_state.get_sprite(player_assets);

    if !timer.0.just_finished() {
        return;
    }

    *sprite_state = sprite_state.advance_frame();
}

#[derive(Component, Debug, Clone, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct AnimationTimer(Timer);

impl AnimationTimer {
    #[tracing::instrument()]
    fn from_seconds(seconds: f32) -> impl Bundle {
        Self(Timer::from_seconds(seconds, TimerMode::Repeating))
    }
}
#[tracing::instrument(skip_all)]
fn tick_animation_timer(mut timers: Query<&mut AnimationTimer>, time: Res<Time>) {
    for mut timer in &mut timers {
        timer.0.tick(time.delta());
    }
}

impl Player {
    #[tracing::instrument()]
    pub fn bundle(player_assets: &PlayerAssets) -> impl Bundle {
        // A texture atlas is a way to split a single image into a grid of related images.
        // You can learn more in this example: https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs
        // let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 6, 2, Some(UVec2::splat(1)), None);
        // let texture_atlas_layout = texture_atlas_layouts.add(layout);
        // let player_animation = PlayerAnimation::new();

        (
            Name::new("Player"),
            Health(20),
            DisplayHealth::bundle(),
            Self,
            CharacterControllerBundle::new(Collider::capsule(48.0, 48.0)),
            Sprite {
                image: player_assets.front1.clone(),
                // color: Color::srgb(3.0, 3.0, 3.0),
                // texture_atlas: Some(TextureAtlas {
                //     layout: texture_atlas_layout,
                //     index: player_animation.get_atlas_index(),
                // }),
                anchor: Anchor::Center,
                ..default()
            },
            PlayerSprite::default(),
            AnimationTimer::from_seconds(5.0 / 60.0),
            Transform::from_scale(Vec2::splat(0.5).extend(1.0)),
            children![
                PlayerAttackIndicator::bundle(player_assets),
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
        )
    }
}

#[tracing::instrument(skip_all)]
fn hurt(
    collisions_with_hurt_box: Single<&CollidingEntities, With<PlayerHurtBox>>,
    health: Single<(&mut Health, Entity), (With<Player>, Without<InvincibilityTimer>)>,
    mut commands: Commands,
) {
    let collisions = *collisions_with_hurt_box;
    let (mut health, entity) = health.into_inner();

    if !collisions.is_empty() {
        log!(Level::Info, "Health: {health:?}");
        health.0 -= 1;

        commands
            .entity(entity)
            .insert(InvincibilityTimer(Timer::from_seconds(
                0.3,
                TimerMode::Once,
            )));

        for &attack_entity in collisions.0.iter() {
            let Ok(mut entity) = commands.get_entity(attack_entity) else {
                continue;
            };

            entity.try_despawn();
        }
    }
}

#[tracing::instrument(skip_all)]
fn handle_player_death(
    query: Option<Single<(), (With<Player>, With<ZeroHealth>)>>,
    next: ResMut<NextState<Screen>>,
) {
    if query.is_none() {
        return;
    }

    log!(Level::Info, "Player Died");
}
