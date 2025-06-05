//! The game's main screen states and transitions between them.

mod gameplay;
mod high_score;
mod loading;
mod splash;
mod title;

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.init_state::<Screen>();

    app.add_plugins((
        gameplay::plugin,
        loading::plugin,
        splash::plugin,
        title::plugin,
        high_score::plugin,
    ));
}

/// The game's main screen states.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[states(scoped_entities)]
pub enum Screen {
    #[default]
    Splash,
    Title,
    Loading,
    Gameplay,
    HighScore,
}
