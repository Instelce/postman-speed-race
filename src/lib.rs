#[cfg(feature = "dev")]
mod dev_tools;
mod game;
mod screen;
mod ui;
mod utils;

use bevy::{
    asset::{load_internal_binary_asset, AssetMetaCheck},
    audio::{AudioPlugin, Volume},
    prelude::*,
    winit::WinitWindows,
};
use bevy_aseprite_ultra::BevySprityPlugin;
use bevy_hanabi::HanabiPlugin;
use utils::get_asset_path;
use winit::window::Icon;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // Order new `AppStep` variants by adding them here:
        app.configure_sets(
            Update,
            (AppSet::TickTimers, AppSet::RecordInput, AppSet::Update),
        );

        // Spawn the main camera.
        app.add_systems(Startup, spawn_camera);

        // Set the game icon.
        app.add_systems(Startup, set_window_icon);

        // Add Bevy plugins.
        app.add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics on web build on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "game".to_string(),
                        canvas: Some("#bevy".to_string()),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: true,
                        ..default()
                    }
                    .into(),
                    ..default()
                })
                .set(AudioPlugin {
                    global_volume: GlobalVolume {
                        volume: Volume::new(0.3),
                    },
                    ..default()
                }),
        );

        // Load custom font.
        load_internal_binary_asset!(
            app,
            TextStyle::default().font,
            "../assets/fonts/minecraftia.ttf",
            |bytes: &[u8], _path: String| { Font::try_from_bytes(bytes.to_vec()).unwrap() }
        );

        // Add external crates plugins.
        app.add_plugins((BevySprityPlugin, HanabiPlugin));

        // Add other plugins
        app.add_plugins((screen::plugin, game::plugin, ui::plugin));

        // Enable dev tools for dev builds.
        #[cfg(feature = "dev")]
        app.add_plugins(dev_tools::plugin);
    }
}

#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum AppSet {
    TickTimers,
    RecordInput,
    Update,
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn set_window_icon(
    // we have to use `NonSend` here
    windows: NonSend<WinitWindows>,
) {
    // here we use the `image` crate to load our icon data from a png file
    // this is not a very bevy-native solution, but it will do
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(get_asset_path("/icons/icon.png"))
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

    // do it for all windows
    for window in windows.windows.values() {
        window.set_window_icon(Some(icon.clone()));
    }
}
