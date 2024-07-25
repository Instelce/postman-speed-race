use bevy::{audio::PlaybackMode, prelude::*};

use crate::game::assets::handles::SfxAssets;

pub(super) fn plugin(app: &mut App) {
    app.observe(play_sfx);
}

fn play_sfx(trigger: Trigger<PlaySfx>, mut commands: Commands, sfx_handles: Res<SfxAssets>) {
    let sfx_key = match trigger.event() {
        PlaySfx::Key(key) => key,
    };
    commands.spawn(AudioSourceBundle {
        source: sfx_handles[sfx_key].clone_weak(),
        settings: PlaybackSettings {
            mode: PlaybackMode::Despawn,
            ..default()
        },
    });
}

/// Trigger this event to play a single sound effect.
#[derive(Event)]
pub enum PlaySfx {
    Key(String),
}
