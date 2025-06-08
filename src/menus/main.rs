//! The main menu (seen on the title screen).

use bevy::prelude::*;

use crate::audio::{
    pause_gameplay_music, pause_not_gameplay_music, resume_not_gameplay_music, spawn_music,
};
use crate::{
    asset_tracking::ResourceHandles, menus::Menu, screens::Screen, theme::widget, AudioSpawned,
};

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(Menu::Main),
        (
            spawn_main_menu,
            pause_gameplay_music,
            resume_not_gameplay_music,
        ),
    );

    app.add_systems(
        Update,
        (spawn_music, pause_gameplay_music, pause_not_gameplay_music)
            .chain()
            .run_if(|res: Res<AudioSpawned>| !res.0),
    );
}

const INSTRUCTIONS: &str = "
WASD to move, left click to parry.

On the high score screen you can type your name.

The score scales exponentially with the max parries, so even though you can spam the parry, it's better to parry a lot of attacks at once. the player's damage also increases according to the number of attacks you parry.
";

fn spawn_main_menu(mut commands: Commands) {
    commands.spawn((
        Name::new("Main Menu"),
        GlobalZIndex(2),
        StateScoped(Menu::Main),
        Node {
            display: Display::Grid,
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(20.0),
            grid_template_columns: vec![RepeatedGridTrack::percent(2, 50.0)],
            ..default()
        },
        Pickable::IGNORE,
        children![
            (
                Node {
                    width: Val::Auto,
                    height: Val::Auto,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    row_gap: Val::Px(20.0),
                    grid_column: GridPlacement::start(1),
                    ..default()
                },
                Text::new(INSTRUCTIONS),
            ),
            (
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(20.0),
                    grid_column: GridPlacement::start(2),
                    ..default()
                },
                // Don't block picking events for other UI roots.
                Pickable::IGNORE,
                #[cfg(not(target_family = "wasm"))]
                children![
                    widget::button("Play", enter_loading_or_gameplay_screen),
                    widget::button("Settings", open_settings_menu),
                    widget::button("Credits", open_credits_menu),
                    widget::button("HighScores", open_high_score_menu),
                    widget::button("Exit", exit_app),
                ],
                #[cfg(target_family = "wasm")]
                children![
                    widget::button("Play", enter_loading_or_gameplay_screen),
                    widget::button("Settings", open_settings_menu),
                    widget::button("Credits", open_credits_menu),
                    widget::button("HighScores", open_high_score_menu),
                ],
            )
        ],
    ));
}

fn enter_loading_or_gameplay_screen(
    _: Trigger<Pointer<Click>>,
    resource_handles: Res<ResourceHandles>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    if resource_handles.is_all_done() {
        next_screen.set(Screen::Gameplay);
    } else {
        next_screen.set(Screen::Loading);
    }
}

fn open_settings_menu(_: Trigger<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Settings);
}

fn open_high_score_menu(_: Trigger<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::HighScore);
}

fn open_credits_menu(_: Trigger<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Credits);
}

#[cfg(not(target_family = "wasm"))]
fn exit_app(_: Trigger<Pointer<Click>>, mut app_exit: EventWriter<AppExit>) {
    app_exit.write(AppExit::Success);
}
