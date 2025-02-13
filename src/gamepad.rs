use bevy::prelude::*;

use crate::player::{
    elements::{PlayerElementState, PressedElements},
    MovementAction,
};

pub struct GamepadPlugin;

impl Plugin for GamepadPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, gamepad_system);
    }
}

fn gamepad_system(
    gamepads: Query<&Gamepad>,
    mut ew_movement: EventWriter<MovementAction>,
    mut element_state: ResMut<NextState<PlayerElementState>>,
    mut pressed_elements: ResMut<PressedElements>,
) {
    for gamepad in &gamepads {
        if gamepad.just_pressed(GamepadButton::South) {
            ew_movement.send(MovementAction::Jump);
        }

        let left_stick_x = gamepad.get(GamepadAxis::LeftStickX).unwrap();
        if !(-0.15..=0.15).contains(&left_stick_x) {
            ew_movement.send(MovementAction::Move(left_stick_x));
        }

        let element_mapping = [
            (GamepadButton::RightTrigger, PlayerElementState::Wind),
            (GamepadButton::RightTrigger2, PlayerElementState::Water),
            (GamepadButton::LeftTrigger, PlayerElementState::Earth),
            (GamepadButton::LeftTrigger2, PlayerElementState::Fire),
        ];

        // Handle new presses
        for (button, element) in element_mapping.iter() {
            if gamepad.just_pressed(*button) {
                if !pressed_elements.0.contains(element) {
                    pressed_elements.0.push(*element);
                }
            }
        }

        // Handle releases
        pressed_elements.0.retain(|element| {
            element_mapping
                .iter()
                .any(|(button, e)| e == element && gamepad.pressed(*button))
        });

        // Set the current element state to the last pressed (top of the stack) or neutral if empty
        element_state.set(
            pressed_elements
                .0
                .last()
                .copied()
                .unwrap_or(PlayerElementState::Neutral),
        );
    }
}
