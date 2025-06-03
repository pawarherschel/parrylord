use crate::screens::Screen;
use crate::{AppSystems, PausableSystems};
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.register_type::<Ttl>();

    app.add_systems(
        Update,
        (
            tick_ttl.in_set(AppSystems::TickTimers),
            get_done_ttl_timers.pipe(handle_done_ttl_timers),
        )
            .run_if(in_state(Screen::Gameplay))
            .in_set(PausableSystems),
    );
}

#[derive(Component, Debug, Clone, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Ttl(Timer);

impl Ttl {
    pub fn new(secs: f32) -> Self {
        Self(Timer::from_seconds(secs, TimerMode::Once))
    }
}

pub fn tick_ttl(mut timers: Query<&mut Ttl>, time: Res<Time>) {
    for mut timer in &mut timers {
        timer.0.tick(time.delta());
    }
}

pub fn get_done_ttl_timers(timers: Query<(&Ttl, Entity)>) -> Vec<Entity> {
    timers
        .iter()
        .filter(|(t, _)| t.0.just_finished())
        .map(|(_, e)| e)
        .collect()
}

pub fn handle_done_ttl_timers(In(timers): In<Vec<Entity>>, mut commands: Commands) {
    for timer in timers {
        commands.entity(timer).despawn();
    }
}
