use avian2d::prelude::LinearVelocity;
use bevy::prelude::*;

use super::{MovementAction, Player};

/// The acceleration used for character movement.
#[derive(Component)]
pub struct MovementAcceleration(pub f32);

/// The damping factor used for slowing down movement.
#[derive(Component)]
pub struct MovementDampingFactor(pub f32);

/// The strength of a jump.
#[derive(Component)]
pub struct JumpImpulse(pub f32);

pub fn process_movement_action(
    mut er: EventReader<MovementAction>,
    mut query: Query<(&mut LinearVelocity, &MovementAcceleration, &JumpImpulse), With<Player>>,
    // time: Res<Time>,
) {
    let Ok((mut linear_velocity, acceleration, jump_impulse)) = query.get_single_mut() else {
        return;
    };

    for event in er.read() {
        match event {
            MovementAction::Move(movement) => {
                linear_velocity.x = movement * acceleration.0;
            }
            MovementAction::Jump => {
                linear_velocity.y = jump_impulse.0;
            }
        }
    }
}

pub fn apply_movement_damping(
    mut query: Query<(&mut LinearVelocity, &MovementDampingFactor), With<Player>>,
    time: Res<Time>,
) {
    let Ok((mut linear_velocity, damping_factor)) = query.get_single_mut() else {
        return;
    };
    linear_velocity.x *= damping_factor.0 * time.delta_secs();
}
