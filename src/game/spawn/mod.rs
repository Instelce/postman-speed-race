use bevy::prelude::*;

pub mod level;
pub mod map;
pub mod player;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((level::plugin, player::plugin, map::plugin));
}
