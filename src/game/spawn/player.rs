use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;

use crate::{
    game::assets::handles::{AsepriteAssets, Handles},
    screen::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.add_event::<SpawnPlayer>();
    app.register_type::<(Player, PlayerController)>();
    app.observe(spawn_player);
}

// NOTE - to trigger this event : `commands.trigger(SpawnPlayer);`
#[derive(Event, Debug)]
pub struct SpawnPlayer;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PlayerController {}

impl Default for PlayerController {
    fn default() -> Self {
        Self {}
    }
}

fn spawn_player(
    _trigger: Trigger<SpawnPlayer>,
    mut commands: Commands,
    aseprite_handles: Res<AsepriteAssets>,
) {
    commands.spawn((
        Name::new("Player"),
        //
        AsepriteAnimationBundle {
            aseprite: aseprite_handles.get("player"),
            animation: Animation::default().with_tag("idle"),
            ..default()
        },
        //
        Player,
        PlayerController::default(),
        //
        StateScoped(Screen::Playing),
    ));
}
