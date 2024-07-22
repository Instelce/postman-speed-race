use bevy::app::App;

pub mod assets;
mod audio;
mod camera;
mod map;
pub mod spawn;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((assets::loaders::plugin, spawn::plugin));
}
