use bevy::prelude::*;

use super::{map::SpawnMap, player::SpawnPlayer};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_level);
}

#[derive(Event, Debug)]
pub struct SpawnLevel(pub i32);

fn spawn_level(trigger: Trigger<SpawnLevel>, mut commands: Commands) {
    commands.trigger(SpawnMap {
        level: trigger.event().0,
    })
}
