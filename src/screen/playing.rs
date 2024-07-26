use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use super::Screen;
use crate::{
    game::{
        audio::soundtrack::PlaySoundtrack,
        circuit::{Circuit, CircuitDuration, EndCircuitTimer},
        house::HouseRotate,
        letter::Letters,
        restart::RestartCooldown,
        spawn::{level::SpawnLevel, map::MapTag},
        ui::{spawn_ui, InfoText},
        GameState,
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

#[derive(Resource, Debug, Default, Clone, Copy, PartialEq, Reflect)]
#[reflect(Resource)]
pub struct CurrentLevel {
    pub indice: i32,
    pub map_size: Vec2,
}

fn enter_playing(
    mut commands: Commands,
    mut clear_color: ResMut<ClearColor>,
    mut camera_query: Query<&mut OrthographicProjection, With<Camera>>,
    current_level: Res<CurrentLevel>,
    mut state: ResMut<NextState<GameState>>,
) {
    state.set(GameState::Run);
    commands.trigger(SpawnLevel(current_level.indice));

    // set background
    clear_color.0 = BACKGROUND;

    // set camera scale
    let mut projection = camera_query.single_mut();
    projection.scale = 0.4;

    commands.init_resource::<Circuit>();
    commands.init_resource::<HouseRotate>();
    commands.init_resource::<EndCircuitTimer>();
    commands.init_resource::<InfoText>();
    commands.init_resource::<CircuitDuration>();
    commands.init_resource::<RestartCooldown>();

    commands.trigger(PlaySoundtrack::Key("Go".into()));
}

fn exit_playing(
    mut commands: Commands,
    mut camera_query: Query<&mut OrthographicProjection, With<Camera>>,
) {
    // reset camera scale
    let mut projection = camera_query.single_mut();
    projection.scale = 1.;

    // remove resources
    commands.remove_resource::<Circuit>();
    commands.remove_resource::<HouseRotate>();
    commands.remove_resource::<Letters>();
    commands.remove_resource::<EndCircuitTimer>();
    commands.remove_resource::<InfoText>();
    commands.remove_resource::<CircuitDuration>();
    commands.remove_resource::<RestartCooldown>();

    commands.trigger(PlaySoundtrack::Disable);
}

fn clear_entities(mut commands: Commands, query: Query<(Entity, &StateScoped<Screen>)>) {
    for (entity, scope) in query.iter() {
        if scope.0 == Screen::Playing {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn return_to_title_screen(mut commands: Commands, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
    commands.trigger(PlaySoundtrack::Key("ChillMenu".into()));
}
