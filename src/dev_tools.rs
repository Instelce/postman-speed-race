//! Development tools for the game. This plugin is only enabled in dev builds.

use bevy::{
    dev_tools::{
        fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
        states::log_transitions,
    },
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::*,
};

use crate::screen::Screen;

pub(super) fn plugin(app: &mut App) {
    // Show FPS
    app.add_plugins((
        FrameTimeDiagnosticsPlugin::default(),
        FpsOverlayPlugin {
            config: FpsOverlayConfig {
                text_config: TextStyle {
                    font_size: 16.,
                    ..default()
                },
            },
        },
    ));
    // Print state transitions in dev builds
    app.add_systems(Update, log_transitions::<Screen>);
}
