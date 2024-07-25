use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_pancam::{PanCam, PanCamPlugin};

use crate::game::camera::MainCamera;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(DebugCameraSettings::default());
    app.add_plugins(PanCamPlugin::default());
    app.add_systems(
        Update,
        (
            toggle_spawn_debug_camera.run_if(input_just_pressed(KeyCode::F1)),
            // camera_movement,
        ),
    );
}

#[derive(Resource)]
pub struct DebugCameraSettings {
    enable: bool,
    last_position: Vec3,
}

impl Default for DebugCameraSettings {
    fn default() -> Self {
        Self {
            enable: false,
            last_position: Vec3::ZERO,
        }
    }
}

#[derive(Component)]
struct DebugCamera;

#[derive(Component)]
pub struct FlyCamera {
    pub acceleration: f32,
    pub max_speed: f32,
    pub friction: f32,
    pub velocity: Vec2,
    pub key_left: KeyCode,
    pub key_right: KeyCode,
    pub key_up: KeyCode,
    pub key_down: KeyCode,
    pub enable: bool,
}

impl Default for FlyCamera {
    fn default() -> Self {
        const MUL_2D: f32 = 10.0;
        Self {
            acceleration: 4. * MUL_2D,
            max_speed: 1. * MUL_2D,
            friction: 1.75 * MUL_2D,
            velocity: Vec2::ZERO,
            key_left: KeyCode::KeyA,
            key_right: KeyCode::KeyD,
            key_up: KeyCode::KeyW,
            key_down: KeyCode::KeyS,
            enable: true,
        }
    }
}

fn toggle_spawn_debug_camera(
    mut commands: Commands,
    mut debug_camera_settings: ResMut<DebugCameraSettings>,
    main_camera: Query<(Entity, &Transform), With<MainCamera>>,
    debug_camera: Query<(Entity, &GlobalTransform), With<DebugCamera>>,
) {
    if debug_camera_settings.enable {
        debug_camera_settings.enable = false;
        // debug_camera_settings.last_position = debug_camera

        // remove debug camera
        let (entity, global_transform) = debug_camera.single();

        debug_camera_settings.last_position = global_transform.translation();
        commands.entity(entity).despawn();

        // add main camera
        let mut camera_bundle = Camera2dBundle::default();
        camera_bundle.projection.scale = 0.4;
        commands.spawn((camera_bundle, MainCamera));
    } else {
        debug_camera_settings.enable = true;

        // remove main camera
        let (entity, transform) = main_camera.single();
        commands.entity(entity).despawn();

        // add debug camera
        commands.spawn((
            Camera2dBundle {
                transform: Transform::from_translation(transform.translation),
                ..default()
            },
            DebugCamera,
            FlyCamera::default(),
            PanCam::default(),
        ));
    }
}

pub fn camera_movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut fly_camera: Query<(&mut FlyCamera, &mut Transform)>,
) {
    if let Ok((mut options, mut transform)) = fly_camera.get_single_mut() {
        let (axis_h, axis_v) = if options.enable {
            (
                movement_axis(&keyboard_input, options.key_right, options.key_left),
                movement_axis(&keyboard_input, options.key_up, options.key_down),
            )
        } else {
            (0., 0.)
        };

        let acceleration = (Vec2::X * axis_h) + (Vec2::Y * axis_v);
        // normalize acceleration
        let acceleration = if acceleration.length() != 0. {
            acceleration.normalize() * options.acceleration
        } else {
            Vec2::ZERO
        };

        let friction = if options.velocity.length() != 0. {
            options.velocity.normalize() * -options.friction
        } else {
            Vec2::ZERO
        };

        options.velocity += acceleration * time.delta_seconds();

        // clamp within max speed
        if options.velocity.length() > options.max_speed {
            options.velocity = options.velocity.normalize() * options.max_speed;
        }

        let delta_friction = friction * time.delta_seconds();

        options.velocity =
            if (options.velocity + delta_friction).signum() != options.velocity.signum() {
                Vec2::ZERO
            } else {
                options.velocity + delta_friction
            };

        transform.translation += Vec3::new(options.velocity.x, options.velocity.y, 0.);
    }
}

// utils

fn movement_axis(input: &Res<ButtonInput<KeyCode>>, plus_key: KeyCode, minus_key: KeyCode) -> f32 {
    let mut axis = 0.;
    if input.pressed(plus_key) {
        axis += 1.;
    }
    if input.pressed(minus_key) {
        axis -= 1.;
    }
    axis
}
