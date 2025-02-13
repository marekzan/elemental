use bevy::prelude::*;

use super::Player;

#[derive(States, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PlayerElementState {
    Neutral,
    Fire,
    Water,
    Earth,
    Wind,
}

#[derive(Resource, Default)]
pub struct PressedElements(pub Vec<PlayerElementState>);

pub fn switch_element(
    element_state: Res<State<PlayerElementState>>,
    mut q_player: Query<&mut Sprite, With<Player>>,
) {
    let Ok(mut sprite) = q_player.get_single_mut() else {
        return;
    };

    match element_state.get() {
        PlayerElementState::Neutral => {
            sprite.color = Color::srgb(0.5, 0.5, 0.5);
        }
        PlayerElementState::Fire => {
            sprite.color = Color::srgb(1.0, 0.0, 0.0);
        }
        PlayerElementState::Water => {
            sprite.color = Color::srgb(0.11, 0.46, 0.87);
        }
        PlayerElementState::Earth => {
            sprite.color = Color::srgb(0.0, 1.0, 0.0);
        }
        PlayerElementState::Wind => {
            sprite.color = Color::srgb(0.9, 0.9, 0.9);
        }
    }
}
