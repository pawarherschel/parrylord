use avian2d::prelude::PhysicsLayer;
use bevy::prelude::*;
mod dynamic_character_2d;
pub mod level;
pub mod player;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        player::plugin,
        level::plugin,
        dynamic_character_2d::CharacterControllerPlugin,
    ));
}

#[derive(PhysicsLayer, Default)]
pub enum CollisionLayer {
    #[default]
    None,
    Walls,
    Player,
    PlayerProjectile,
    Enemy,
    EnemyProjectile,
}
