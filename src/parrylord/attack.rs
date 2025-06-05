use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.register_type::<Attack>();
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Attack(pub u8);
