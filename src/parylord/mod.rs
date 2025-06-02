use crate::asset_tracking::LoadResource;
use avian2d::prelude::PhysicsLayer;
use bevy::prelude::*;

pub mod dynamic_character_2d;
pub mod enemy;
pub mod enemy_attack;
pub mod health;
pub mod level;
pub mod player;
pub mod player_attack;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        player::plugin,
        level::plugin,
        dynamic_character_2d::CharacterControllerPlugin,
        enemy::plugin,
        health::plugin,
        player_attack::plugin,
        enemy_attack::plugin,
    ));

    app.load_resource::<LevelAssets>();

    app.register_type::<EnemyAssets>();
    app.load_resource::<EnemyAssets>();

    app.register_type::<PlayerAssets>();
    app.load_resource::<PlayerAssets>();
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
    #[dependency]
    attack: Handle<Image>,
}

impl EnemyAssets {
    pub const MAX_ASSETS: u8 = 4;
}

impl FromWorld for EnemyAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();

        Self {
            beige: assets.load("images/enemy/beige.png"),
            blue: assets.load("images/enemy/blue.png"),
            green: assets.load("images/enemy/green.png"),
            yellow: assets.load("images/enemy/yellow.png"),
            attack: assets.load("images/enemy_attack/star_07.png"),
        }
    }
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct PlayerAssets {
    #[dependency]
    pub pink: Handle<Image>,
    #[dependency]
    pub attack_indicator: Handle<Image>,
}

impl FromWorld for PlayerAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            pink: assets.load("images/pink/front.png"),

            attack_indicator: assets.load("images/pink/attack_indicator.png"),
        }
    }
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct LevelAssets {
    #[dependency]
    pub bg: Handle<Image>,
}

impl FromWorld for LevelAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            bg: assets.load("images/bg.png"),
        }
    }
}
