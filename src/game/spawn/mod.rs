use bevy::prelude::*;

mod level;
mod player;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((level::plugin, player::plugin));
}
