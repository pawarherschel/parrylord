use avian2d::prelude::PhysicsLayer;
use bevy::prelude::*;

pub mod assets;
mod attack;
pub mod dynamic_character_2d;
pub mod enemy;
pub mod enemy_attack;
pub mod health;
pub mod level;
pub mod player;
pub mod player_attack;
pub mod ttl;

pub fn plugin(app: &mut App) {
    app.init_resource::<ParrylordLevel>();

    app.add_plugins((
        assets::plugin,
        attack::plugin,
        player::plugin,
        level::plugin,
        dynamic_character_2d::plugin,
        enemy::plugin,
        health::plugin,
        player_attack::plugin,
        enemy_attack::plugin,
        ttl::plugin,
    ));
}

#[derive(PhysicsLayer, Default)]
pub enum CollisionLayer {
    #[default]
    None,
    Walls,
    Player,
    PlayerHurt,
    PlayerProjectile,
    PlayerParry,
    Enemy,
    EnemyProjectile,
}

#[derive(Resource, Clone, Reflect, Debug, Default)]
#[reflect(Resource)]
pub struct ParrylordLevel(u8);
