use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    // app.register_type::<TTL>();
    //
    // app.add_systems(
    //     Update,
    //     (tick_ttl.in_set(AppSystems::TickTimers), remove_on_ttl)
    //         .run_if(in_state(Screen::Gameplay))
    //         .in_set(PausableSystems),
    // );
}

// #[derive(Component, Debug, Clone, PartialEq, Eq, Default, Reflect)]
// #[reflect(Component)]
// pub struct TTL(Timer);
//
// impl TTL {
//     pub fn new(secs: f32) -> Self {
//         Self(Timer::from_seconds(secs, TimerMode::Once))
//     }
// }
//
// pub fn tick_ttl(mut timer: Query<(&mut TTL)>) {}
//
// pub fn remove_on_ttl() {}
