use std::f32::consts::PI;

use bevy::{prelude::*, window::PrimaryWindow};

use crate::screen::Screen;

use super::circuit::{Circuit, CircuitDirection, CircuitOrientation};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<CameraTarget>();
    app.add_systems(FixedUpdate, follow_target.run_if(in_state(Screen::Playing)));
    app.add_systems(Update, resizing);
}

#[derive(Component, Reflect)]
pub struct MainCamera;

#[derive(Component, Reflect)]
pub struct CameraTarget;

fn follow_target(
    time: Res<Time>,
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
    target_query: Query<&Transform, (With<CameraTarget>, Without<MainCamera>)>,
    circuit: Res<Circuit>,
) {
    if let Ok(mut camera_transform) = camera_query.get_single_mut() {
        if let Ok(target_transform) = target_query.get_single() {
            let mut offset = if circuit.in_turn {
                Vec3::ZERO
            } else if circuit.current_orientation == CircuitOrientation::Vertical {
                Vec3::Y * 50.
            } else if circuit.current_orientation == CircuitOrientation::Horizontal {
                Vec3::X * 50.
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
            } else if circuit.turn_count >= 2 || circuit.turn_count == -1 {
                -offset
            } else {
                offset
            };

            camera_transform.translation = camera_transform.translation.lerp(
                target_transform.translation + offset,
                time.delta_seconds() * 12.,
            );
            // camera_transform.rotation = camera_transform.rotation.lerp(
            //     Quat::from_axis_angle(Vec3::Z, reset_angle),
            //     time.delta_seconds() * 5.,
            // );
            camera_transform.rotation = camera_transform
                .rotation
                .lerp(target_transform.rotation, time.delta_seconds() * 4.);
        }
    }
}

fn resizing(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut camera_query: Query<&mut OrthographicProjection, With<MainCamera>>,
) {
    if !window_query.is_empty() && !camera_query.is_empty() {
        let window = window_query.single();
        let mut projection = camera_query.single_mut();

        let start_scale = 0.4;
        let start_width = 1280.;

        if start_width == window.width().ceil() {
            projection.scale = start_scale;
        } else {
            projection.scale = start_scale * (start_width / window.width());
        }
    }
}
