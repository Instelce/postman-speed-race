//! Development tools for the game. This plugin is only enabled in dev builds.

use bevy_inspector_egui::DefaultInspectorConfigPlugin;
pub use bevy_pancam::PanCam;

use bevy::{
    dev_tools::{
        fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
        states::log_transitions,
    },
    diagnostic::FrameTimeDiagnosticsPlugin,
    input::common_conditions::input_toggle_active,
    prelude::*,
    prelude::*,
};
use debug_camera::*;
use inspector::inspector_ui;

use crate::screen::Screen;

mod debug_camera;
mod inspector;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        DefaultInspectorConfigPlugin,
        bevy_egui::EguiPlugin,
        // Show FPS
        FrameTimeDiagnosticsPlugin::default(),
        FpsOverlayPlugin {
            config: FpsOverlayConfig {
                text_config: TextStyle {
                    font_size: 16.,
                    ..default()
                },
            },
        },
        // Debug camera
        debug_camera::plugin,
    ));

    // Debug context
    app.register_type::<DebugContext>();
    app.init_resource::<DebugContext>();

    // Systems
    app.add_systems(
        Update,
        (
            toggle_debug_context,
            inspector_ui.run_if(input_toggle_active(false, KeyCode::F1)),
        ),
    );

    // Print state transitions in dev builds
    app.add_systems(Update, log_transitions::<Screen>);
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct DebugContext {
    pub enabled: bool,
}

impl Default for DebugContext {
    fn default() -> Self {
        Self { enabled: true }
    }
}

fn toggle_debug_context(keys: Res<ButtonInput<KeyCode>>, mut debug_context: ResMut<DebugContext>) {
    if keys.just_pressed(KeyCode::Semicolon) {
        debug_context.enabled = !debug_context.enabled;
    }
}
