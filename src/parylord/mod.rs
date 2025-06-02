use avian2d::prelude::PhysicsLayer;
use bevy::prelude::*;
mod dynamic_character_2d;
pub mod enemy;
mod health;
pub mod level;
pub mod player;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        player::plugin,
        level::plugin,
        dynamic_character_2d::CharacterControllerPlugin,
        enemy::plugin,
        health::plugin,
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
    Enemy,
    EnemyProjectile,
}
