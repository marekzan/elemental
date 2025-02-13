use bevy::{pbr::wireframe::WireframePlugin, prelude::*};
use fps::FpsPlugin;

mod fps;
mod wireframe;

#[derive(Resource)]
pub struct DebugOptions {
    pub fps: bool,
    pub wireframe: bool,
}

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DebugOptions {
            fps: false,
            wireframe: false,
        })
        .add_plugins((WireframePlugin, FpsPlugin))
        .add_systems(Update, activate_debug_options);
    }
}

fn activate_debug_options(mut options: ResMut<DebugOptions>, input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::KeyF) {
        info!("Toggling fps");
        options.fps = !options.fps;
    }
    if input.just_pressed(KeyCode::KeyW) {
        info!("Toggling wireframe");
        options.wireframe = !options.wireframe;
    }
}
