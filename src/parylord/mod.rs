use bevy::prelude::*;
mod player;
mod movement;
pub(crate) mod level;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((player::plugin, movement::plugin, level::plugin));
}
