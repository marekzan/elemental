pub mod elements;
pub mod movement;
mod spawn;

use bevy::prelude::*;
use elements::{switch_element, PlayerElementState, PressedElements};
use movement::{apply_movement_damping, process_movement_action};
use spawn::setup;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MovementAction>()
            .insert_state(PlayerElementState::Fire)
            .init_resource::<PressedElements>()
            .add_systems(Startup, setup)
            .add_systems(
                FixedUpdate,
                (process_movement_action, apply_movement_damping),
            )
            .add_systems(Update, switch_element);
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Event)]
pub enum MovementAction {
    Move(f32),
    Jump,
}
