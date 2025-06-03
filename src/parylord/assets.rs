use crate::asset_tracking::LoadResource;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.register_type::<LevelAssets>();
    app.load_resource::<LevelAssets>();

    app.register_type::<EnemyAssets>();
    app.load_resource::<EnemyAssets>();

    app.register_type::<PlayerAssets>();
    app.load_resource::<PlayerAssets>();
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct EnemyAssets {
    #[dependency]
    pub beige: Handle<Image>,
    #[dependency]
    pub blue: Handle<Image>,
    #[dependency]
    pub green: Handle<Image>,
    #[dependency]
    pub yellow: Handle<Image>,
    #[dependency]
    pub attack: Handle<Image>,
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

// #[derive(Resource, Asset, Clone, Debug, Reflect)]
// #[reflect(Resource)]
// pub struct Shaders {
//     tint_shader: TintMaterial,
// }
//
// #[derive(AsBindGroup, Debug, Clone, Asset, Reflect)]
// pub struct TintMaterial {
//     #[texture(0)]
//     #[sampler(1)]
//     color_texture: Handle<Image>,
// }
//
// impl Material2d for TintMaterial {
//     fn fragment_shader() -> ShaderRef {
//         "shaders/tint_material.wgsl".into()
//     }
// }
