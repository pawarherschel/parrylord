use crate::screens::Screen;
use crate::{AppSystems, PausableSystems};
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.register_type::<Health>();
    app.register_type::<ZeroHealth>();
    app.register_type::<InvincibilityTimer>();
    app.register_type::<DisplayHealth>();

    app.add_systems(
        Update,
        (
            tick_invincibility_timer.in_set(AppSystems::TickTimers),
            change_invinsibile_visibility,
            despawn_done_invincibility_timers,
            check_health.pipe(handle_health),
            display_health,
        )
            .run_if(in_state(Screen::Gameplay))
            .in_set(PausableSystems),
    );
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Health(pub u8);

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct ZeroHealth;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct DisplayHealth;

impl DisplayHealth {
    pub fn bundle() -> impl Bundle {
        (
            Self,
            Text2d::new("meow"),
            TextColor(Color::BLACK),
            TextFont {
                font_size: 90.0,
                ..default()
            },
            Visibility::Visible,
        )
    }
}

pub fn display_health(mut healths: Query<(&mut Text2d, &Health), With<DisplayHealth>>) {
    for (mut display_health, health) in &mut healths {
        display_health.0 = format!("{}", health.0);
    }
}

#[derive(Component, Debug, Clone, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct InvincibilityTimer(pub Timer);

pub fn check_health(healths: Query<(&Health, Entity)>) -> Vec<Entity> {
    healths
        .iter()
        .filter(|(health, ..)| health.0 <= 0)
        .map(|(_, entity)| entity)
        .collect()
}

pub fn handle_health(In(entities): In<Vec<Entity>>, mut commands: Commands) {
    for entity in entities {
        commands
            .entity(entity)
            .remove::<Health>()
            .remove::<InvincibilityTimer>()
            .insert(ZeroHealth);
    }
}

pub fn tick_invincibility_timer(mut timers: Query<&mut InvincibilityTimer>, time: Res<Time>) {
    for mut timer in &mut timers {
        timer.0.tick(time.delta());
    }
}

const CHANGE_TIME_THING: f32 = 0.5;

pub fn change_invinsibile_visibility(mut query: Query<(&mut Visibility, &InvincibilityTimer)>) {
    for (mut vis, timer) in &mut query {
        let time_remaining = timer.0.remaining_secs();
        let fract = (time_remaining % CHANGE_TIME_THING) / CHANGE_TIME_THING;

        if fract > 0.3 {
            *vis = Visibility::Hidden;
        } else {
            *vis = Visibility::Inherited;
        }
    }
}

pub fn despawn_done_invincibility_timers(
    mut timers: Query<(&InvincibilityTimer, Entity, &mut Visibility)>,
    mut commands: Commands,
) {
    for (timer, entity, mut vis) in &mut timers {
        if timer.0.just_finished() {
            *vis = Visibility::Inherited;
            commands.entity(entity).remove::<InvincibilityTimer>();
        }
    }
}
