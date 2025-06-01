use crate::asset_tracking::LoadResource;
use crate::parylord::player::{player, PlayerAssets};
use crate::screens::Screen;
use bevy::image::{ImageLoaderSettings, ImageSampler};
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<LevelAssets>();
    app.load_resource::<LevelAssets>();
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct LevelAssets {
    #[dependency]
    bg: Handle<Image>,
}

impl FromWorld for LevelAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            bg: assets.load("images/bg.png"),
        }
    }
}

/// A system that spawns the main level.
pub fn spawn_level(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    player_assets: Res<PlayerAssets>,
) {
    commands.spawn((
        Name::new("Level"),
        Transform::default(),
        Visibility::default(),
        StateScoped(Screen::Gameplay),
        children![
            player(600.0, &player_assets),
            (
                Sprite {
                    image: level_assets.bg.clone(),
                    ..default()
                },
                Transform::from_xyz(0.0, 0.0, -1000.0)
            )
        ],
    ));
}
