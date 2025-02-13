use bevy::{prelude::*, sprite::Wireframe2dPlugin};

#[cfg(not(target_arch = "wasm32"))]
use bevy::sprite::Wireframe2dConfig;

pub struct WireframePlugin;

impl Plugin for WireframePlugin {
    fn build(&self, app: &mut App) {
        #[cfg(not(target_arch = "wasm32"))]
        app.add_plugins(Wireframe2dPlugin)
            .add_systems(Update, toggle_wireframe);
    }
}

pub fn toggle_wireframe(
    mut wireframe_config: ResMut<Wireframe2dConfig>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        wireframe_config.global = !wireframe_config.global;
    }
}
