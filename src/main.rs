mod camera;
mod debug;
mod gamepad;
mod physics;
mod player;

use crate::{
    camera::CameraPlugin, debug::DebugPlugin, gamepad::GamepadPlugin, physics::PhysicsPlugin,
};
use bevy::{
    prelude::*,
    window::{PresentMode, WindowMode, WindowResolution},
};
use player::PlayerPlugin;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins.set(windowplugin_config()).build(),
        PhysicsPlugin,
        GamepadPlugin,
        PlayerPlugin,
        CameraPlugin,
        DebugPlugin,
    ));

    app.insert_resource(ClearColor(Color::WHITE));

    app.run();
}

fn windowplugin_config() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            present_mode: PresentMode::Immediate,
            mode: WindowMode::Windowed,
            fit_canvas_to_parent: false,
            canvas: Some("#game-canvas".to_string()),
            resolution: WindowResolution::new(1920.0, 1024.0),
            ..default()
        }),
        ..default()
    }
}
