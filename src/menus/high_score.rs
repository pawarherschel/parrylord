use crate::asset_tracking::ResourceHandles;
use crate::menus::Menu;
use crate::screens::Screen;
use crate::theme::palette::LABEL_TEXT;
use crate::theme::widget;
use crate::zaphkiel::has_bad_word;
use crate::{HighScore, HighScores, ParrylordSingleton, CF_WORKER_URL};
use bevy::prelude::*;
use bevy_mod_reqwest::{BevyReqwest, ReqwestErrorEvent};
use std::cmp::PartialEq;

pub fn plugin(app: &mut App) {
    app.init_resource::<NameField>();
    app.add_systems(OnEnter(Menu::HighScore), spawn_high_score);
    // app.add_systems(
    //     Update,
    //     spawn_high_score
    //         .run_if(resource_changed::<NameField>)
    //         .run_if(in_state(Menu::HighScore)),
    // );
    app.add_systems(
        Update,
        (update_name, tick_inactive_timer).run_if(in_state(Menu::HighScore)),
    );
}

fn spawn_high_score(
    mut commands: Commands,
    singleton: Res<ParrylordSingleton>,
    scores: Res<HighScores>,
) {
    let ParrylordSingleton {
        enemies_killed,
        level,
        max_parried,
    } = *singleton;
    let score = singleton.calculate_score();

    let scores = scores.0.clone();

    let root = commands.spawn(widget::ui_root("High Score")).id();

    let Ok(mut root) = commands.get_entity(root) else {
        warn!("Ok(mut root) = commands.get_entity(root): {root:?}");
        return;
    };

    root.insert((
        GlobalZIndex(2),
        StateScoped(Menu::HighScore),
        children![
            (
                widget::label("Name: "),
                children![(
                    TextSpan::new("<Start Typing>"),
                    TextThing,
                    TextFont::from_font_size(24.0),
                    TextColor(LABEL_TEXT),
                )]
            ),
            widget::header(format!("Score: {score}")),
            widget::label(format!("Enemies Killed: {enemies_killed}")),
            widget::label(format!("Level Reached: {level}")),
            widget::label(format!("Max Projectiles Parried: {max_parried}")),
            widget::button("Submit Score", submit_score),
            widget::button("Play Again", enter_loading_or_gameplay_screen),
        ],
    ));

    root.with_children(|children_spawner| {
        for (pos, HighScore { name, score }) in
            (0..10).map(|idx| (idx + 1, scores.get(idx).cloned().unwrap_or_default()))
        {
            children_spawner.spawn(widget::label(format!("{pos}: {name} -> {score}")));
        }

        children_spawner.spawn(widget::button("Main Menu", open_main_menu));
    });

    commands.insert_resource(Inactive(Timer::from_seconds(1.0, TimerMode::Once)));
}

fn enter_loading_or_gameplay_screen(
    _: Trigger<Pointer<Click>>,
    resource_handles: Res<ResourceHandles>,
    mut next_screen: ResMut<NextState<Screen>>,
    inactive: Option<Res<Inactive>>,
) {
    if inactive.is_some() {
        return;
    }

    if resource_handles.is_all_done() {
        next_screen.set(Screen::Gameplay);
    } else {
        next_screen.set(Screen::Loading);
    }
}

fn open_main_menu(
    _: Trigger<Pointer<Click>>,
    mut next_menu: ResMut<NextState<Menu>>,
    inactive: Option<Res<Inactive>>,
) {
    if inactive.is_some() {
        return;
    }

    next_menu.set(Menu::Main);
}

#[derive(Component)]
struct TextThing;

#[derive(Resource, Debug, Eq, PartialEq)]
struct NameField(String);

impl Default for NameField {
    fn default() -> Self {
        Self(String::from("<Start Typing>"))
    }
}

#[derive(Resource)]
struct Inactive(Timer);

fn tick_inactive_timer(
    mut timer: Option<ResMut<Inactive>>,
    time: Res<Time>,
    mut commands: Commands,
) {
    let Some(mut timer) = timer else {
        return;
    };

    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        commands.remove_resource::<Inactive>();
    }
}

fn submit_score(
    _: Trigger<Pointer<Click>>,
    singleton: Res<ParrylordSingleton>,
    mut name_field: ResMut<NameField>,
    mut next_menu: ResMut<NextState<Menu>>,
    mut client: BevyReqwest,
    inactive: Option<Res<Inactive>>,
) {
    if inactive.is_some() {
        return;
    }

    if has_bad_word(&name_field.0) {
        name_field.0 = String::from("BAD WORD DETECTED");
        return;
    }

    let score = singleton.calculate_score();

    if *name_field == NameField::default()
        || score == 1
        || name_field
            .0
            .as_str()
            .eq_ignore_ascii_case("BAD WORD DETECTED")
    {
        return;
    }

    let reqwest_request = client
        .post(CF_WORKER_URL)
        .json(&HighScore {
            name: name_field.0.clone(),
            score,
        })
        .build()
        .unwrap();

    client
        .send(reqwest_request)
        .on_error(|trigger: Trigger<ReqwestErrorEvent>| {
            let e = &trigger.event().0;
            error!(?e);
        });

    next_menu.set(Menu::Main);
}

fn update_name(
    key: Res<ButtonInput<KeyCode>>,
    mut name_field: ResMut<NameField>,
    mut text_thing: Single<&mut TextSpan, With<TextThing>>,
    inactive: Option<Res<Inactive>>,
) {
    if inactive.is_some() {
        return;
    }

    let keys_pressed = key.get_just_pressed().collect::<Vec<_>>();

    if (*name_field == NameField::default()
        || name_field
            .0
            .as_str()
            .eq_ignore_ascii_case("BAD WORD DETECTED"))
        && !keys_pressed.is_empty()
    {
        name_field.0 = String::new();
    }

    for &key in keys_pressed {
        if key == KeyCode::Backspace {
            let new_len = name_field.0.len().saturating_sub(1);
            name_field.0.truncate(new_len);
        } else {
            name_field.0.push(match key {
                KeyCode::Digit0 => '0',
                KeyCode::Digit1 => '1',
                KeyCode::Digit2 => '2',
                KeyCode::Digit3 => '3',
                KeyCode::Digit4 => '4',
                KeyCode::Digit5 => '5',
                KeyCode::Digit6 => '6',
                KeyCode::Digit7 => '7',
                KeyCode::Digit8 => '8',
                KeyCode::Digit9 => '9',
                KeyCode::KeyA => 'A',
                KeyCode::KeyB => 'B',
                KeyCode::KeyC => 'C',
                KeyCode::KeyD => 'D',
                KeyCode::KeyE => 'E',
                KeyCode::KeyF => 'F',
                KeyCode::KeyG => 'G',
                KeyCode::KeyH => 'H',
                KeyCode::KeyI => 'I',
                KeyCode::KeyJ => 'J',
                KeyCode::KeyK => 'K',
                KeyCode::KeyL => 'L',
                KeyCode::KeyM => 'M',
                KeyCode::KeyN => 'N',
                KeyCode::KeyO => 'O',
                KeyCode::KeyP => 'P',
                KeyCode::KeyQ => 'Q',
                KeyCode::KeyR => 'R',
                KeyCode::KeyS => 'S',
                KeyCode::KeyT => 'T',
                KeyCode::KeyU => 'U',
                KeyCode::KeyV => 'V',
                KeyCode::KeyW => 'W',
                KeyCode::KeyX => 'X',
                KeyCode::KeyY => 'Y',
                KeyCode::KeyZ => 'Z',
                KeyCode::Space => ' ',
                _ => continue,
            });
        }
    }

    if name_field.0.is_empty() {
        *name_field = NameField::default();
    }

    (***text_thing).clone_from(&name_field.0);
}
