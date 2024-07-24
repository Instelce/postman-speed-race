use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::{Animation, AnimationRepeat};
use bevy_hanabi::{velocity, EffectProperties, EffectSpawner};

use crate::screen::Screen;

use super::{
    movements::Velocity,
    spawn::player::{Player, PlayerController, PlayerMovement, PlayerParticles},
};

/// Animations and particles
pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, animate_player.run_if(in_state(Screen::Playing)));
}

fn animate_player(
    mut query: Query<
        (
            &Transform,
            &mut Animation,
            &PlayerMovement,
            &PlayerController,
            &Velocity,
        ),
        With<Player>,
    >,
    mut particles_query: Query<(&mut EffectSpawner, &mut EffectProperties), With<PlayerParticles>>,
) {
    if query.is_empty() || particles_query.is_empty() {
        return;
    }

    let (transform, mut animation, movement, controller, velocity) = query.single_mut();
    let (mut spawner, mut properties) = particles_query.single_mut();

    if animation.tag != Some("launch-letter".into()) {
        if velocity.0.length() < 0.2 && velocity.0.length() > -0.2 {
            animation.play("pause", AnimationRepeat::Loop);
        } else if movement.friction >= 12. {
            animation.play("brake", AnimationRepeat::Loop);
        }

        // particles
        if velocity.0.length() > 0.4 {
            properties.set("velocity", (movement.direction.extend(0.) * -4.).into());
            spawner.reset();
        }

        if velocity.0.length() > 2. {
            animation.play("ride-fast", AnimationRepeat::Loop);
        } else if velocity.0.length() > 0.2 {
            animation.play("ride", AnimationRepeat::Loop);
        }
    }
}
