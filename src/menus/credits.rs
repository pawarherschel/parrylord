//! The credits menu.

use crate::{menus::Menu, theme::prelude::*};
use bevy::{ecs::spawn::SpawnIter, input::common_conditions::input_just_pressed, prelude::*};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Credits), spawn_credits_menu);
    app.add_systems(
        Update,
        go_back.run_if(in_state(Menu::Credits).and(input_just_pressed(KeyCode::Escape))),
    );

    // app.add_systems(OnEnter(Menu::Credits), start_credits_music);
}

fn spawn_credits_menu(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Credits Menu"),
        GlobalZIndex(2),
        StateScoped(Menu::Credits),
        children![
            widget::header("Created by"),
            created_by(),
            widget::header("Assets"),
            assets(),
            widget::button("Back", go_back_on_click),
        ],
    ));
}

fn created_by() -> impl Bundle {
    grid(vec![
        ["Herschel Pawar", "Solo dev"],
        [
            "The Bevy Flock",
            "Project Scaffolding ('bevy_new_2d' Template)",
        ],
        ["Jondolf", "Dynamic Character Controller starting point"],
    ])
}

fn assets() -> impl Bundle {
    grid(vec![
        ["Sprites", "CC0 by kenney.nl"],
        ["Button SFX", "CC0 by Jaszunio15"],
        [
            "Bevy logo",
            "All rights reserved by the Bevy Foundation, permission granted for splash screen use when unmodified",
        ],
        ["Bad words list", "techpulsetoday"],
    ])
}

fn grid(content: Vec<[&'static str; 2]>) -> impl Bundle {
    (
        Name::new("Grid"),
        Node {
            display: Display::Grid,
            row_gap: Val::Px(10.0),
            column_gap: Val::Px(30.0),
            grid_template_columns: RepeatedGridTrack::px(2, 400.0),
            ..default()
        },
        Children::spawn(SpawnIter(content.into_iter().flatten().enumerate().map(
            |(i, text)| {
                (
                    widget::label(text),
                    Node {
                        justify_self: if i % 2 == 0 {
                            JustifySelf::End
                        } else {
                            JustifySelf::Start
                        },
                        ..default()
                    },
                )
            },
        ))),
    )
}

fn go_back_on_click(_: Trigger<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Main);
}

fn go_back(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Main);
}

// fn start_credits_music(mut commands: Commands, credits_music: Res<CreditsAssets>) {
//     commands.spawn((
//         Name::new("Credits Music"),
//         StateScoped(Menu::Credits),
//         music(credits_music.music.clone()),
//     ));
// }
