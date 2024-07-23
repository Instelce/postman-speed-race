use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use super::Screen;
use crate::{
    game::{
        circuit::Circuit,
        house::HouseRotate,
        letter::Letters,
        spawn::{level::SpawnLevel, map::MapTag},
    },
    ui::palette::BACKGROUND,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Playing), enter_playing);
    app.add_systems(OnExit(Screen::Playing), exit_playing);

    app.add_systems(
        Update,
        return_to_title_screen
            .run_if(in_state(Screen::Playing).and_then(input_just_pressed(KeyCode::Escape))),
    );
}

fn enter_playing(
    mut commands: Commands,
    mut clear_color: ResMut<ClearColor>,
    mut camera_query: Query<&mut OrthographicProjection, With<Camera>>,
) {
    commands.trigger(SpawnLevel);

    // set background
    clear_color.0 = BACKGROUND;

    // set camera scale
    let mut projection = camera_query.single_mut();
    projection.scale = 0.4;

    commands.init_resource::<Circuit>();
    commands.init_resource::<HouseRotate>();
}

fn exit_playing(
    mut commands: Commands,
    mut camera_query: Query<&mut OrthographicProjection, With<Camera>>,
    map: Query<Entity, With<MapTag>>,
) {
    // reset camera scale
    let mut projection = camera_query.single_mut();
    projection.scale = 1.;

    // despawn map
    let map = map.single();
    commands.entity(map).despawn_recursive();

    // remove resources
    commands.remove_resource::<Circuit>();
    commands.remove_resource::<HouseRotate>();
    commands.remove_resource::<Letters>();
}

fn return_to_title_screen(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}
