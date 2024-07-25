use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::{Animation, AnimationRepeat, AsepriteAnimationBundle};
use rand::Rng;

use crate::{screen::Screen, AppSet};

use super::{
    assets::handles::AsepriteAssets,
    circuit::{Circuit, CircuitDirection},
    collider::Collider,
    map::chunk::PIXEL_CHUNK_SIZE,
    restart::Restart,
    spawn::{
        map::{ChunkTag, FollowPlayerRotation, ObstacleTag, PostOffice},
        player::{Player, PlayerController},
    },
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<HouseOrientation>();
    app.add_systems(
        Update,
        ((rotate_house, follow_player_rotation, obstacle_check).in_set(AppSet::Update),)
            .run_if(in_state(Screen::Playing)),
    );
}

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct HouseRotate(pub bool);

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct HouseOrientation {
    pub angle_mul: i32,
}

pub fn rotate_house(
    mut house_query: Query<(&mut Transform, &mut HouseOrientation)>,
    circuit: Res<Circuit>,
    mut house_rotate: ResMut<HouseRotate>,
) {
    if circuit.direction_chosen && !house_rotate.0 {
        for (mut transform, mut orientation) in house_query.iter_mut() {
            let rotation = (PI / 2.) * orientation.angle_mul as f32;
            if circuit.direction == CircuitDirection::AntiClockwise {
                transform.rotate_z(rotation);
            }
            if circuit.direction == CircuitDirection::Clockwise {
                transform.rotate_z(-rotation);
            }

            if orientation.angle_mul == 0 {
                transform.translation += Vec3::X * 16.;
            } else if orientation.angle_mul == 1 {
                transform.translation -= Vec3::Y * 16.;
            } else if orientation.angle_mul == 2 {
                transform.translation += Vec3::X * PIXEL_CHUNK_SIZE - 32.;
            } else if orientation.angle_mul == 3 {
                transform.translation -= Vec3::Y * PIXEL_CHUNK_SIZE - 32.;
            }

            transform.translation.z = 0.05;
        }

        house_rotate.0 = true;
    }
}

fn follow_player_rotation(
    time: Res<Time>,
    circuit: Res<Circuit>,
    mut query: Query<&mut Transform, With<FollowPlayerRotation>>,
) {
    for mut transform in query.iter_mut() {
        let mut reset_angle = match circuit.direction {
            CircuitDirection::AntiClockwise => PI,
            CircuitDirection::Clockwise => 0.,
        };

        let turn = (circuit.turn_count) as f32 * PI / 2.;

        reset_angle += match circuit.direction {
            CircuitDirection::AntiClockwise => -turn,
            CircuitDirection::Clockwise => -turn,
        };

        transform.rotation = transform.rotation.lerp(
            Quat::from_axis_angle(Vec3::Z, reset_angle),
            time.delta_seconds() * 30.,
        );
        transform.translation.z = 0.05;
    }
}

fn obstacle_check(
    time: Res<Time>,
    mut commands: Commands,
    mut player_query: Query<(&Collider, &mut Animation, &mut PlayerController), With<Player>>,
    obstacles_query: Query<&Collider, (With<ObstacleTag>, Without<Player>, Without<ChunkTag>)>,
) {
    if let Ok((player_collider, mut animation, mut controller)) = player_query.get_single_mut() {
        controller.start_timer.tick(time.delta());
        if !controller.damn && controller.start_timer.finished() {
            for (obstacle_collider) in obstacles_query.iter() {
                if player_collider.collide(obstacle_collider) {
                    controller.damn = true;
                    animation.play("fall", AnimationRepeat::Count(0));
                }
            }
        }

        if controller.damn && animation.tag != Some("fall".into()) {
            commands.trigger(Restart);
        }
    }
}
