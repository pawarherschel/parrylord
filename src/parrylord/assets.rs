use crate::asset_tracking::LoadResource;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.register_type::<LevelAssets>();
    app.load_resource::<LevelAssets>();

    app.register_type::<EnemyAssets>();
    app.load_resource::<EnemyAssets>();

    app.register_type::<PlayerAssets>();
    app.load_resource::<PlayerAssets>();

    app.register_type::<AttackAssets>();
    app.load_resource::<AttackAssets>();
}

#[derive(Resource, Asset, Clone, Reflect, Debug)]
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
        }
    }
}

#[derive(Resource, Asset, Clone, Reflect, Debug)]
#[reflect(Resource)]
pub struct PlayerAssets {
    #[dependency]
    pub front2: Handle<Image>,
    #[dependency]
    pub front1: Handle<Image>,
    #[dependency]
    pub walk1: Handle<Image>,
    #[dependency]
    pub walk2: Handle<Image>,
    #[dependency]
    pub stand: Handle<Image>,
    #[dependency]
    pub attack_indicator: Handle<Image>,
}

impl FromWorld for PlayerAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            front1: assets.load("images/pink/front1.png"),
            front2: assets.load("images/pink/front2.png"),
            walk1: assets.load("images/pink/walk1.png"),
            walk2: assets.load("images/pink/walk2.png"),
            stand: assets.load("images/pink/stand.png"),
            attack_indicator: assets.load("images/pink/attack_indicator.png"),
        }
    }
}

#[derive(Resource, Asset, Clone, Reflect, Debug)]
#[reflect(Resource)]
pub struct AttackAssets {
    #[dependency]
    pub _0: Handle<Image>,
    #[dependency]
    pub _1: Handle<Image>,
    #[dependency]
    pub _2: Handle<Image>,
    #[dependency]
    pub _3: Handle<Image>,
    #[dependency]
    pub _4: Handle<Image>,
    #[dependency]
    pub _5: Handle<Image>,
    #[dependency]
    pub _6: Handle<Image>,
    #[dependency]
    pub _7: Handle<Image>,
    #[dependency]
    pub _8: Handle<Image>,
    #[dependency]
    pub _9: Handle<Image>,
    #[dependency]
    pub _10: Handle<Image>,
    #[dependency]
    pub _11: Handle<Image>,
}

impl AttackAssets {
    pub const MAX: u8 = 12;
}

impl FromWorld for AttackAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();

        Self {
            _0: assets.load("images/attack/0.png"),
            _1: assets.load("images/attack/1.png"),
            _2: assets.load("images/attack/2.png"),
            _3: assets.load("images/attack/3.png"),
            _4: assets.load("images/attack/4.png"),
            _5: assets.load("images/attack/5.png"),
            _6: assets.load("images/attack/6.png"),
            _7: assets.load("images/attack/7.png"),
            _8: assets.load("images/attack/8.png"),
            _9: assets.load("images/attack/9.png"),
            _10: assets.load("images/attack/10.png"),
            _11: assets.load("images/attack/11.png"),
        }
    }
}

#[derive(Resource, Asset, Clone, Reflect, Debug)]
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
