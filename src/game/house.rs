use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::{Animation, AsepriteAnimationBundle};
use rand::Rng;

use crate::screen::Screen;

use super::{
    assets::handles::AsepriteAssets,
    circuit::{Circuit, CircuitDirection},
    map::chunk::PIXEL_CHUNK_SIZE,
    spawn::map::PostOffice,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<HouseOrientation>();
    app.add_systems(
        Update,
        (rotate_house, post_office_letter_spawn).run_if(in_state(Screen::Playing)),
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

            transform.translation += Vec3::Z * 0.05;
        }

        house_rotate.0 = true;
    }
}

fn post_office_letter_spawn(
    mut commands: Commands,
    chunk_query: Query<&Transform, With<PostOffice>>,
    mut is_spawn: Local<bool>,
    aseprites: Res<AsepriteAssets>,
) {
    if !*is_spawn {
        if let Ok(transform) = chunk_query.get_single() {
            commands
                .spawn((
                    Name::new("Post Office Letters"),
                    SpatialBundle {
                        transform: Transform::from_translation(transform.translation),
                        ..default()
                    },
                    StateScoped(Screen::Playing),
                ))
                .with_children(|children| {
                    let mut rng = rand::thread_rng();
                    for _ in 0..100 {
                        let translation = Vec2::new(
                            rng.gen_range(0..PIXEL_CHUNK_SIZE as u32) as f32,
                            -(rng.gen_range(0..PIXEL_CHUNK_SIZE as u32) as f32) + 16.,
                        );
                        let angle = rng.gen_range(1..6) as f32 - 0.25;
                        let scale = Vec2::splat(rng.gen_range(6..10) as f32 / 10.);

                        let tag = if rng.gen_ratio(2, 10) {
                            "craft"
                        } else {
                            "small"
                        };

                        children.spawn(
                            (AsepriteAnimationBundle {
                                aseprite: aseprites.get("letter"),
                                animation: Animation::default().with_tag(tag),
                                transform: Transform::from_translation(translation.extend(0.))
                                    .with_rotation(Quat::from_axis_angle(Vec3::Z, angle))
                                    .with_scale(scale.extend(0.)),
                                ..default()
                            }),
                        );
                    }
                });

            *is_spawn = true;
        }
    }
}
