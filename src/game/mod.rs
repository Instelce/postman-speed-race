use bevy::app::App;

pub mod animation;
pub mod assets;
mod audio;
mod camera;
pub mod circuit;
pub mod collider;
pub mod house;
pub mod letter;
mod map;
pub mod movements;
pub mod save;
pub mod spawn;
pub mod ui;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        save::plugin,
        assets::loaders::plugin,
        spawn::plugin,
        ui::plugin,
        animation::plugin,
        camera::plugin,
        movements::plugin,
        circuit::plugin,
        collider::plugin,
        house::plugin,
        letter::plugin,
    ));
}
