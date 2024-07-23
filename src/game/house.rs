use std::f32::consts::PI;

use bevy::prelude::*;

use crate::screen::Screen;

use super::{
    circuit::{Circuit, CircuitDirection},
    map::chunk::PIXEL_CHUNK_SIZE,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<HouseOrientation>();
    app.add_systems(Update, rotate_house.run_if(in_state(Screen::Playing)));
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
