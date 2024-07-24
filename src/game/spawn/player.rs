use std::f32::consts::PI;

use bevy::{math::VectorSpace, prelude::*};
use bevy_aseprite_ultra::prelude::*;
use bevy_hanabi::{ParticleEffect, ParticleEffectBundle};

use crate::{
    game::{
        assets::handles::{AsepriteAssets, ParticleEffectAssets},
        camera::CameraTarget,
        collider::Collider,
        movements::Velocity,
    },
    screen::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.add_event::<SpawnPlayer>();
    app.register_type::<(Player, PlayerController)>();
    app.observe(spawn_player);
}

// NOTE - to trigger this event : `commands.trigger(SpawnPlayer);`
#[derive(Event, Debug)]
pub struct SpawnPlayer(pub Vec2);

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PlayerParticles;

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PlayerController {
    pub can_launch_letter: bool,
    pub letter_target: Option<Entity>,
    pub closest_launch_zone: Option<Collider>,
    pub letter_launched: bool,
    pub actual_collision: Option<Collider>,
    pub actual_chunk: Option<Collider>,
    pub end_timer: Timer,
}

impl Default for PlayerController {
    fn default() -> Self {
        Self {
            can_launch_letter: false,
            letter_target: None,
            closest_launch_zone: None,
            letter_launched: false,
            actual_collision: None,
            actual_chunk: None,
            end_timer: Timer::from_seconds(1., TimerMode::Once),
        }
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PlayerMovement {
    pub direction: Vec2,
    pub factor: f32,
    pub acceleration: f32,
    pub max_speed: f32,
    pub friction: f32,
    pub dash_mul: f32,
}

impl Default for PlayerMovement {
    fn default() -> Self {
        const MULT: f32 = 4.; // multiplier
        Self {
            direction: Vec2::ZERO,
            factor: 0.,
            acceleration: 8. * MULT,
            max_speed: 1. * MULT,
            friction: 2.,
            // friction: 2.25 * MULT,
            dash_mul: 2.,
        }
    }
}

fn spawn_player(
    trigger: Trigger<SpawnPlayer>,
    mut commands: Commands,
    aseprite_handles: Res<AsepriteAssets>,
    particles: Res<ParticleEffectAssets>,
) {
    commands
        .spawn((
            Name::new("Player"),
            //
            AsepriteAnimationBundle {
                aseprite: aseprite_handles.get("postman"),
                animation: Animation::default().with_tag("ride"),
                transform: Transform::from_translation(trigger.event().0.extend(0.1))
                    .with_rotation(Quat::from_axis_angle(Vec3::Z, -PI / 2.))
                    .with_scale(Vec3::new(1.5, 1.5, 0.)),
                ..default()
            },
            //
            Player,
            PlayerController::default(),
            PlayerMovement::default(),
            Velocity::default(),
            Collider::rect(5., 5.),
            //
            CameraTarget,
            //
            StateScoped(Screen::Playing),
        ))
        .with_children(|children| {
            children.spawn((
                ParticleEffectBundle {
                    effect: ParticleEffect::new(particles.get("player")),
                    transform: Transform::from_translation(Vec3::new(0., -9., 0.)),
                    ..default()
                },
                PlayerParticles,
            ));
        });
}
