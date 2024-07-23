use bevy::prelude::*;

use crate::screen::Screen;

use super::circuit::{Circuit, CircuitDirection, CircuitOrientation};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<CameraTarget>();
    app.add_systems(FixedUpdate, follow_target.run_if(in_state(Screen::Playing)));
}

#[derive(Component, Reflect)]
pub struct CameraTarget;

fn follow_target(
    time: Res<Time>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    target_query: Query<&Transform, (With<CameraTarget>, Without<Camera>)>,
    circuit: Res<Circuit>,
) {
    if let Ok(mut camera_transform) = camera_query.get_single_mut() {
        if let Ok(target_transform) = target_query.get_single() {
            let mut offset = if circuit.in_turn {
                Vec3::ZERO
            } else if circuit.current_orientation == CircuitOrientation::Vertical {
                Vec3::Y * 30.
            } else if circuit.current_orientation == CircuitOrientation::Horizontal {
                Vec3::X * 30.
            } else {
                Vec3::ZERO
            };

            offset = if circuit.direction == CircuitDirection::Clockwise {
                offset
            } else {
                -offset
            };

            offset = if circuit.turn_count >= 4 {
                offset
            } else if circuit.turn_count >= 2 {
                -offset
            } else {
                offset
            };

            camera_transform.translation = camera_transform.translation.lerp(
                target_transform.translation + offset,
                time.delta_seconds() * 12.,
            );
            camera_transform.rotation = camera_transform
                .rotation
                .lerp(target_transform.rotation, time.delta_seconds() * 5.);
        }
    }
}
