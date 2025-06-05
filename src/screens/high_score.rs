use crate::menus::Menu;
use crate::screens::Screen;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::HighScore), open_high_score);
    app.add_systems(OnExit(Screen::HighScore), close_high_score);
}

fn open_high_score(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::HighScore);
}

fn close_high_score(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}
