//! The screen state for the main gameplay.

use crate::audio::{pause_not_gameplay_music, resume_gameplay_music};
use crate::{menus::Menu, parrylord::level::spawn_level, screens::Screen, Pause};
use avian2d::prelude::{Physics, PhysicsTime};
use bevy::{input::common_conditions::input_just_pressed, prelude::*};

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(Screen::Gameplay),
        (spawn_level, pause_not_gameplay_music, resume_gameplay_music),
    );

    // Toggle pause on key press.
    app.add_systems(
        Update,
        (
            (pause, spawn_pause_overlay, open_pause_menu).run_if(
                in_state(Screen::Gameplay)
                    .and(in_state(Menu::None))
                    .and(input_just_pressed(KeyCode::KeyP).or(input_just_pressed(KeyCode::Escape))),
            ),
            close_menu.run_if(
                in_state(Screen::Gameplay)
                    .and(not(in_state(Menu::None)))
                    .and(input_just_pressed(KeyCode::KeyP)),
            ),
        ),
    );
    app.add_systems(
        OnExit(Screen::Gameplay),
        (
            close_menu,
            unpause,
            // pause_gameplay_music,
            // resume_not_gameplay_music,
        ),
    );
    app.add_systems(
        OnEnter(Menu::None),
        unpause.run_if(in_state(Screen::Gameplay)),
    );
}

fn unpause(mut next_pause: ResMut<NextState<Pause>>, mut physics: ResMut<Time<Physics>>) {
    next_pause.set(Pause(false));
    physics.unpause();
}

fn pause(mut next_pause: ResMut<NextState<Pause>>, mut physics: ResMut<Time<Physics>>) {
    next_pause.set(Pause(true));
    physics.pause();
}

fn spawn_pause_overlay(mut commands: Commands) {
    commands.spawn((
        Name::new("Pause Overlay"),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        GlobalZIndex(1),
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
        StateScoped(Pause(true)),
    ));
}

fn open_pause_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Pause);
}

fn close_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}
