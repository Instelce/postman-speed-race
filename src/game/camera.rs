use bevy::prelude::*;

use crate::screen::Screen;

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
) {
    if let Ok(mut camera_transform) = camera_query.get_single_mut() {
        if let Ok(target_transform) = target_query.get_single() {
            camera_transform.translation = camera_transform.translation.lerp(
                target_transform.translation + Vec3::Y * 30.,
                time.delta_seconds() * 20.,
            );
            camera_transform.rotation = camera_transform
                .rotation
                .lerp(target_transform.rotation, time.delta_seconds() * 5.);
        }
    }
}
