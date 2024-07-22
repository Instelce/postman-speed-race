use bevy::app::App;

pub mod assets;
mod audio;
mod camera;
pub mod circuit;
pub mod collider;
mod map;
pub mod movements;
pub mod spawn;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        assets::loaders::plugin,
        spawn::plugin,
        camera::plugin,
        movements::plugin,
        circuit::plugin,
        collider::plugin,
    ));
}
