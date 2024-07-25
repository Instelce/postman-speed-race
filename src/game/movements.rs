use std::f32::consts::PI;

use bevy::{math::VectorSpace, prelude::*};

use crate::{screen::Screen, AppSet};

use super::{
    circuit::{Circuit, CircuitDirection, EndCircuitTimer},
    collider::Collider,
    spawn::{
        map::{ChunkTag, NotRoadTile},
        player::{Player, PlayerController, PlayerMovement},
    },
    GameState,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        (
            player_movements.in_set(AppSet::RecordInput),
            (update_entities_transform, off_the_road).in_set(AppSet::Update),
        )
            .run_if(in_state(Screen::Playing)),
    );

    app.add_systems(
        Update,
        (clear_movement_end.run_if(in_state(GameState::End))),
    );
}

#[derive(Component, Reflect, Debug, Default, Deref, DerefMut)]
#[reflect(Component)]
pub struct Velocity(pub Vec2);

pub fn player_movements(
    time: Res<Time>,
    end_timer: Res<EndCircuitTimer>,
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<
        (
            &mut Transform,
            &mut Velocity,
            &mut PlayerMovement,
            &mut PlayerController,
        ),
        With<Player>,
    >,
    mut circuit: ResMut<Circuit>,
) {
    if let Ok((mut transform, mut velocity, mut movement, _)) = query.get_single_mut() {
        if end_timer.elapsed_secs() > 1. {
            return;
        }

        let mut rotation_factor = 0.;

        if !(end_timer.elapsed_secs() > 0.) {
            // vertical axis
            if keys.pressed(KeyCode::KeyW) {
                movement.factor = 1.;
            } else {
                movement.factor = 0.;
            }
        } else {
            movement.factor = 0.;
        }

        // brake
        if keys.pressed(KeyCode::KeyS) {
            movement.friction = 12.;
        } else if movement.friction == 12. {
            movement.friction = 2.;
        }

        // horizontal axis
        if keys.pressed(KeyCode::KeyD) {
            // right
            rotation_factor -= 1.;
            movement.direction.x = -0.5;
        } else if keys.pressed(KeyCode::KeyA) {
            // left
            rotation_factor += 1.;
            movement.direction.x = 0.5;
        } else {
            movement.direction.x = 0.;
        }

        if circuit.direction_chosen && !circuit.in_turn {
            // reset player rotation to the circuit direction

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
                time.delta_seconds() * 10.,
            );
        } else if !circuit.direction_chosen {
            // set circuit direction when the player start
            if rotation_factor > 0. {
                circuit.direction = CircuitDirection::Clockwise;
                circuit.direction_chosen = true;
            } else if rotation_factor < 0. {
                circuit.direction = CircuitDirection::Clockwise; // haha no AntiClockwise
                circuit.direction_chosen = true;
            }
        }

        // rotate player
        if movement.direction.y != 0. {
            let speed = if circuit.in_turn { 6.5 } else { 5. };

            if rotation_factor == 0. {
            } else {
                transform.rotate_z(rotation_factor * speed * time.delta_seconds());
            }
        }

        let movement_direction = transform.rotation * Vec3::Y;

        if movement.factor != 0. {
            movement.direction = movement_direction.xy() * movement.factor;
        } else {
            movement.direction = Vec2::ZERO;
        }
        // let movement_distance = movement.factor * 60. * time.delta_seconds();

        // transform.translation += movement_direction * movement_distance;
        // return;

        let acceleration = if movement.direction.length() != 0. {
            movement.direction.normalize() * movement.acceleration
        } else {
            Vec2::ZERO
        };

        let friction = if velocity.length() != 0. {
            velocity.0 * -1. * movement.friction
        } else {
            Vec2::ZERO
        };

        velocity.0 += acceleration * time.delta_seconds();

        if velocity.length() > movement.max_speed {
            velocity.0 = velocity.normalize() * movement.max_speed;
        }

        let delta_friction = friction * time.delta_seconds();

        velocity.0 = if (velocity.0 + delta_friction).signum() != velocity.signum() {
            Vec2::ZERO
        } else {
            velocity.0 + delta_friction
        };

        // if controller.dashing {
        //     // println!("dash ...");
        //     controller.dash_timer.tick(time.delta());
        //     velocity.0 *= movement.dash_mul;
        // }

        // if controller.dash_timer.finished() && controller.dashing {
        //     // println!("Dash finished");
        //     controller.dashing = false;
        //     controller.dash_cooldown_timer.reset();
        // }
    }
}

fn update_entities_transform(mut query: Query<(&mut Transform, &mut Velocity)>) {
    for (mut transform, mut velocity) in query.iter_mut() {
        let delta_x = velocity.x;
        let delta_y = velocity.y;

        // apply x transform
        transform.translation += Vec3::new(delta_x, 0., 0.);

        // check collision on the x axis

        // apply y transform
        transform.translation += Vec3::new(0., delta_y, 0.);

        // check collision on the y axis
    }
}

fn off_the_road(
    mut player_query: Query<(&mut PlayerMovement, &mut PlayerController, &Collider), With<Player>>,
    chunk_query: Query<(&Children, &Collider), With<ChunkTag>>,
    tile_query: Query<(&Parent, &Collider), With<NotRoadTile>>,
) {
    if let Ok((mut movement, mut controller, player_collider)) = player_query.get_single_mut() {
        for (children, chunk_collider) in chunk_query.iter() {
            if player_collider.collide(chunk_collider) {
                controller.actual_chunk = Some(chunk_collider.clone());

                for child in children.iter() {
                    if let Ok((parent, tile_collider)) = tile_query.get(*child) {
                        if player_collider.collide(tile_collider) {
                            controller.actual_collision = Some(tile_collider.clone());
                        }
                    }
                }

                break;
            }
        }

        if let Some(collider) = &controller.actual_collision {
            movement.friction = 20.;

            if !player_collider.collide(collider) {
                movement.friction = 2.;
                controller.actual_collision = None;
            }
        }

        if let Some(collider) = &controller.actual_chunk {
            if !player_collider.collide(collider) {
                movement.friction = 20.;
                controller.actual_chunk = None;
            }
        }
    }
}

fn clear_movement_end(
    end_timer: Res<EndCircuitTimer>,
    mut player_query: Query<(&mut Transform, &mut Velocity, &mut PlayerController), With<Player>>,
) {
    if let Ok((mut transform, mut velocity, _)) = player_query.get_single_mut() {
        if end_timer.elapsed_secs() > 1. {
            velocity.0 = Vec2::splat(0.);
        }

        transform.rotation = Quat::from_axis_angle(Vec3::Z, 0.);
    }
}
