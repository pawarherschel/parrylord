// https://github.com/Jondolf/avian/blob/main/crates/avian2d/examples/dynamic_character_2d/plugin.rs

use crate::parylord::CollisionLayer;
use crate::screens::Screen;
use crate::{exponential_decay, PausableSystems};
use avian2d::math::{AdjustPrecision, Scalar, Vector};
use avian2d::prelude::*;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_event::<MovementAction>();

    app.add_systems(
        Update,
        (keyboard_input, movement)
            .chain()
            .run_if(in_state(Screen::Gameplay))
            .in_set(PausableSystems),
    );
}

/// An event sent for a movement input action.
#[derive(Event)]
pub enum MovementAction {
    Move(Vector),
    None,
}

/// A marker component indicating that an entity is using a character controller.
#[derive(Component)]
pub struct CharacterController;

/// The acceleration used for character movement.
#[derive(Component)]
pub struct MaxMovementSpeed(Scalar);

/// A bundle that contains the components needed for a basic
/// kinematic character controller.
#[derive(Bundle)]
pub struct CharacterControllerBundle {
    character_controller: CharacterController,
    body: RigidBody,
    collider: Collider,
    collision_layer: CollisionLayers,
    locked_axes: LockedAxes,
    movement: MovementBundle,
}

/// A bundle that contains components for character movement.
#[derive(Bundle)]
pub struct MovementBundle {
    max_speed: MaxMovementSpeed,
}

impl MovementBundle {
    pub const fn new(speed: Scalar) -> Self {
        Self {
            max_speed: MaxMovementSpeed(speed),
        }
    }
}

impl Default for MovementBundle {
    fn default() -> Self {
        Self::new(300.0)
    }
}

impl CharacterControllerBundle {
    pub fn new(collider: Collider) -> Self {
        // Create shape caster as a slightly smaller version of collider
        let mut caster_shape = collider.clone();
        caster_shape.set_scale(Vector::ONE * 0.99, 10);

        Self {
            character_controller: CharacterController,
            body: RigidBody::Dynamic,
            collider,
            collision_layer: CollisionLayers::new(
                [CollisionLayer::Player],
                [CollisionLayer::Walls],
            ),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            movement: MovementBundle::default(),
        }
    }
}

/// Sends [`MovementAction`] events based on keyboard input.
fn keyboard_input(
    mut movement_event_writer: EventWriter<MovementAction>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let left = keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
    let right = keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);
    let up = keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]);
    let down = keyboard_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]);

    let horizontal = f32::from(right as i8 - left as i8);
    let vertical = f32::from(up as i8 - down as i8);
    let direction = Vector::new(horizontal, vertical);

    movement_event_writer.write(if direction.length_squared() != 0.0 {
        MovementAction::Move(direction)
    } else {
        MovementAction::None
    });
}

/// Responds to [`MovementAction`] events and moves character controllers accordingly.
fn movement(
    time: Res<Time>,
    mut movement_event_reader: EventReader<MovementAction>,
    mut controllers: Query<(&MaxMovementSpeed, &mut LinearVelocity)>,
) {
    // Precision is adjusted so that the example works with
    // both the `f32` and `f64` features. Otherwise you don't need this.
    let delta_time = time.delta_secs_f64().adjust_precision();

    for event in movement_event_reader.read() {
        for (max_speed, mut linear_velocity) in &mut controllers {
            match event {
                MovementAction::Move(direction) => {
                    let curr = linear_velocity.0;
                    let to_add = *direction * max_speed.0;
                    let target = curr + to_add;

                    let actual =
                        exponential_decay!(current: curr, target: target, delta: delta_time)
                            .clamp_length_max(max_speed.0);

                    *linear_velocity = LinearVelocity(actual);
                }
                MovementAction::None => {
                    let curr = linear_velocity.0;
                    let target = Vector::ZERO;

                    let actual =
                        exponential_decay!(current: curr, target: target, decay: 30.0 ,delta: delta_time)
                            .clamp_length_min(0.0);

                    *linear_velocity = LinearVelocity(actual);
                }
            }
        }
    }
}
