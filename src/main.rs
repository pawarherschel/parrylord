// Support configuring Bevy lints within code.
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]
extern crate core;

mod asset_tracking;
mod audio;
// mod demo;
pub mod assets;
#[cfg(feature = "dev")]
mod dev_tools;
mod menus;
mod parrylord;
mod screens;
mod theme;
mod zaphkiel;

use crate::assets::{GameplayMusic, MusicAudio, NotGameplayMusic};
use crate::audio::music;
use avian2d::prelude::Gravity;
use avian2d::PhysicsPlugins;
use bevy::time::common_conditions::on_timer;
use bevy::window::WindowResolution;
use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_mod_reqwest::{BevyReqwest, JsonResponse, ReqwestErrorEvent, ReqwestPlugin};
use std::time::Duration;

fn main() -> AppExit {
    App::new().add_plugins(AppPlugin).run()
}

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // Add Bevy plugins.
        app.add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics on web build on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "Parrylord".to_string(),
                        fit_canvas_to_parent: true,
                        resolution: WindowResolution::new(1920.0, 1080.0),
                        ..default()
                    }
                    .into(),
                    ..default()
                }),
        );

        // Add other plugins.
        app.add_plugins((
            asset_tracking::plugin,
            audio::plugin,
            // demo::plugin,
            parrylord::plugin,
            #[cfg(feature = "dev")]
            dev_tools::plugin,
            menus::plugin,
            screens::plugin,
            theme::plugin,
            ReqwestPlugin::default(),
        ));

        app.add_plugins(PhysicsPlugins::default())
            .insert_resource(Gravity::ZERO);

        // Order new `AppSystems` variants by adding them here:
        app.configure_sets(
            Update,
            (
                AppSystems::TickTimers,
                AppSystems::RecordInput,
                AppSystems::Update,
            )
                .chain(),
        );

        // Set up the `Pause` state.
        app.init_state::<Pause>();
        app.configure_sets(Update, PausableSystems.run_if(in_state(Pause(false))));

        // Spawn the main camera.
        app.add_systems(Startup, spawn_camera);

        app.init_resource::<ParrylordSingleton>();
        app.init_resource::<HighScores>();

        app.add_systems(
            Update,
            get_high_scores.run_if(on_timer(Duration::from_secs_f32(10.0))),
        );

        app.init_resource::<AudioSpawned>();
    }
}

/// High-level groupings of systems for the app in the `Update` schedule.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum AppSystems {
    /// Tick timers.
    TickTimers,
    /// Record player input.
    RecordInput,
    /// Do everything else (consider splitting this into further variants).
    Update,
}

/// Whether or not the game is paused.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[states(scoped_entities)]
struct Pause(pub bool);

/// A system set for systems that shouldn't run while the game is paused.
#[derive(SystemSet, Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct PausableSystems;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Camera2d,
        // Camera {
        //     hdr: true, // 1. HDR is required for bloom
        //     clear_color: ClearColorConfig::Custom(Color::BLACK),
        //     ..default()
        // },
        // Tonemapping::ReinhardLuminance, // 2. Using a tonemapper that desaturates to white is recommended
        // Bloom::default(),               // 3. Enable bloom for the camera
        // DebandDither::Enabled,          // Optional: bloom causes gradients which cause banding
    ));
}

#[derive(Resource, Clone, Reflect, Debug)]
#[reflect(Resource)]
pub struct ParrylordSingleton {
    pub enemies_killed: u32,
    pub level: u32,
    pub max_parried: u32,
}

impl Default for ParrylordSingleton {
    fn default() -> Self {
        Self {
            enemies_killed: 0,
            level: 1,
            max_parried: 0,
        }
    }
}

impl ParrylordSingleton {
    #[must_use]
    pub const fn calculate_score(&self) -> u128 {
        let &Self {
            enemies_killed,
            level,
            max_parried,
        } = self;

        let enemies_killed = enemies_killed as u128;
        let level = level as u128;
        let max_parried = max_parried;

        (level + enemies_killed).saturating_pow(max_parried)
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Default)]
pub struct HighScore {
    pub name: String,
    pub score: u128,
}

#[derive(serde::Deserialize, Debug, Clone, Default, Resource)]
pub struct HighScores(pub Vec<HighScore>);

pub const CF_WORKER_URL: &str = "https://parrylord-high-score-worker.pawarherschel.workers.dev/";

fn get_high_scores(mut client: BevyReqwest) {
    let reqwest_request = client.get(CF_WORKER_URL).build().unwrap();

    client
        .send(reqwest_request)
        .on_json_response(
            |trigger: Trigger<JsonResponse<HighScores>>, mut high_scores: ResMut<HighScores>| {
                let data = trigger.0.clone();

                high_scores.0 = data.0;
            },
        )
        .on_error(|trigger: Trigger<ReqwestErrorEvent>| {
            let e = &trigger.event().0;
            error!(?e);
        });
}

#[derive(Resource, Default)]
pub struct AudioSpawned(pub(crate) bool);
