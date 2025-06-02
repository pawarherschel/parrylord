use crate::screens::Screen;
use crate::{AppSystems, PausableSystems};
use bevy::ecs::component::HookContext;
use bevy::ecs::system::entity_command::{insert, remove};
use bevy::ecs::world::DeferredWorld;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.register_type::<Health>();
    app.register_type::<ZeroHealth>();

    app.add_systems(
        Update,
        (
            tick_invincibility_timer.in_set(AppSystems::TickTimers),
            despawn_done_invincibility_timers,
        )
            .run_if(in_state(Screen::Gameplay))
            .in_set(PausableSystems),
    )
    .add_systems(
        Update,
        check_health
            .pipe(handle_health)
            .run_if(in_state(Screen::Gameplay))
            .in_set(PausableSystems),
    );
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Health(pub i8);

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct ZeroHealth;

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

pub fn tick_invincibility_timer(mut timers: Query<(&mut InvincibilityTimer)>, time: Res<Time>) {
    for mut timer in &mut timers {
        timer.0.tick(time.delta());
    }
}

pub fn despawn_done_invincibility_timers(
    timers: Query<(&InvincibilityTimer, Entity)>,
    mut commands: Commands,
) {
    for (timer, entity) in &timers {
        if timer.0.just_finished() {
            commands.entity(entity).remove::<InvincibilityTimer>();
        }
    }
}
