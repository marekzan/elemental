use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use log::info;

use super::DebugOptions;

pub struct FpsPlugin;

impl Plugin for FpsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(fps_plugin_config())
            .add_systems(Update, (toggle_fps, log_fps));
    }
}

pub fn fps_plugin_config() -> FpsOverlayPlugin {
    FpsOverlayPlugin {
        config: FpsOverlayConfig {
            text_config: TextFont {
                font: default(),
                font_size: 20.0,
                font_smoothing: bevy::text::FontSmoothing::default(),
            },
            text_color: Color::srgb(0.0, 1.0, 0.0),
            enabled: false,
        },
    }
}
pub fn toggle_fps(input: Res<ButtonInput<KeyCode>>, mut overlay: ResMut<FpsOverlayConfig>) {
    if input.just_pressed(KeyCode::Space) {
        overlay.enabled = !overlay.enabled;
    }
}

fn log_fps(diagnostic: Res<DiagnosticsStore>, debug_options: Res<DebugOptions>) {
    if let Some(fps_information) = diagnostic.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if debug_options.fps {
            let fps = fps_information.value().unwrap_or_default();
            if fps < 50.0 {
                info!("fps: {:?}", fps)
            }
        }
    }
}
