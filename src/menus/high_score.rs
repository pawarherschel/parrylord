use crate::menus::main::enter_loading_or_gameplay_screen;
use crate::menus::Menu;
use crate::theme::widget;
use crate::ParrylordSingleton;
use bevy::app::App;
use bevy::asset::AssetContainer;
use bevy::ecs::children;
use bevy::prelude::SpawnRelated;
use bevy::prelude::{Commands, GlobalZIndex, OnEnter, Res, StateScoped};
use log::{error, info, warn};
use serde::Deserialize;
use std::sync::mpsc;
use std::sync::mpsc::RecvError;
use std::thread::spawn;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::HighScore), spawn_high_score);
}

fn spawn_high_score(mut commands: Commands, singleton: Res<ParrylordSingleton>) {
    let ParrylordSingleton {
        enemies_killed,
        level,
        max_parried,
    } = *singleton;
    let score = singleton.calculate_score();

    let scores = get_high_scores();

    let root = commands.spawn(widget::ui_root("High Score")).id();

    // TODO: wasm crashes on entering this screen for some reason.

    let Ok(mut root) = commands.get_entity(root) else {
        warn!("Ok(mut root) = commands.get_entity(root): {root:?}");
        return;
    };

    root.insert((
        GlobalZIndex(2),
        StateScoped(Menu::HighScore),
        children![
            widget::header(format!("Score: {score}")),
            widget::label(format!("Enemies Killed: {enemies_killed}")),
            widget::label(format!("Level Reached: {level}")),
            widget::label(format!("Max Projectiles Parried: {max_parried}")),
            widget::button("Play Again", enter_loading_or_gameplay_screen),
        ],
    ));

    root.with_children(|children_spawner| {
        for (pos, HighScore { name, score }) in scores.iter().enumerate() {
            children_spawner.spawn(widget::label(format!("{pos}: {name} -> {score}")));
        }
    });
}

const URL: &str = "https://parrylord-high-score-worker.pawarherschel.workers.dev/";

fn get_high_scores() -> Vec<HighScore> {
    let request = ehttp::Request::get(URL);

    let (send, recv) = mpsc::channel();

    ehttp::fetch(request, move |result: ehttp::Result<ehttp::Response>| {
        let Ok(res) = result else {
            error!("fetch request failed");
            send.send(None).unwrap();
            return;
        };
        info!("status code: {}", res.status);
        if res.status == 404 {
            error!("got: {:?}", res.text());
            send.send(None).unwrap();
            return;
        }

        let Ok(json) = res.json::<HighScores>() else {
            error!("unable to get json");
            send.send(None).unwrap();
            return;
        };

        let Err(x) = send.send(Some(json)) else {
            send.send(None).unwrap();
            return;
        };
        error!("Unable to send data: {x:?}");
        send.send(None).unwrap();
    });

    match recv.recv() {
        Ok(Some(x)) => x.0,
        x => {
            error!("{x:?}");
            Vec::new()
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Default)]
struct HighScore {
    name: String,
    score: u64,
}

#[derive(serde::Deserialize, Debug, Clone, Default)]
struct HighScores(Vec<HighScore>);
