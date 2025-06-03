use crate::asset_tracking::LoadResource;
use crate::exponential_decay;
use crate::parylord::assets::PlayerAssets;
use crate::parylord::dynamic_character_2d::CharacterControllerBundle;
use crate::parylord::health::{DisplayHealth, Health, InvincibilityTimer, ZeroHealth};
use crate::parylord::player_attack::PlayerAttackIndicator;
use crate::parylord::CollisionLayer;
use crate::screens::Screen;
use crate::{AppSystems, PausableSystems};
use avian2d::prelude::{Collider, CollidingEntities, CollisionLayers, Sensor};
use bevy::prelude::*;
use bevy::sprite::Anchor;
use log::{log, Level};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Player>();

    // Record directional input as movement controls.
    app.add_systems(
        Update,
        (hurt, handle_player_death)
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
pub struct PlayerHurtBox;
impl Player {
    pub fn spawn(player_assets: &PlayerAssets) -> impl Bundle {
        // A texture atlas is a way to split a single image into a grid of related images.
        // You can learn more in this example: https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs
        // let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 6, 2, Some(UVec2::splat(1)), None);
        // let texture_atlas_layout = texture_atlas_layouts.add(layout);
        // let player_animation = PlayerAnimation::new();

        (
            Name::new("Player"),
            Health(20),
            DisplayHealth::spawn(),
            Self,
            CharacterControllerBundle::new(Collider::capsule(48.0, 48.0)),
            Sprite {
                image: player_assets.pink.clone(),
                // color: Color::srgb(3.0, 3.0, 3.0),
                // texture_atlas: Some(TextureAtlas {
                //     layout: texture_atlas_layout,
                //     index: player_animation.get_atlas_index(),
                // }),
                anchor: Anchor::Center,
                ..default()
            },
            Transform::from_scale(Vec2::splat(0.5).extend(1.0)),
            children![
                PlayerAttackIndicator::spawn(player_assets),
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

fn hurt(
    collisions_with_hurt_box: Single<(&CollidingEntities), With<PlayerHurtBox>>,
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
    }
}

fn handle_player_death(query: Option<Single<(), (With<Player>, With<ZeroHealth>)>>) {
    if query.is_none() {
        return;
    }

    log!(Level::Info, "Player Died");
}
