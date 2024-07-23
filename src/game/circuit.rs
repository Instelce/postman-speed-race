use std::default;

use bevy::{color::palettes::css::BLUE, prelude::*};

use crate::{screen::Screen, AppSet};

use super::{
    collider::{collide, Collider, Collision},
    map::chunk::ChunkConnextion,
    spawn::{
        map::{ChunkConnextions, ChunkRoad, ChunkTag},
        player::Player,
    },
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Circuit>();
    app.add_systems(
        Update,
        update_circuit
            .in_set(AppSet::Update)
            .run_if(in_state(Screen::Playing)),
    );
}

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
pub struct Circuit {
    pub current_orientation: CircuitOrientation,
    pub turn_count: i32,
    #[reflect(ignore)]
    pub turn: Vec<Entity>,
    #[reflect(ignore)]
    pub already_collide: Vec<Entity>,
    pub in_turn: bool,
    pub direction: CircuitDirection,
    pub direction_chosen: bool,
}

#[derive(Debug, Reflect, Default, PartialEq, Eq)]
pub enum CircuitDirection {
    #[default]
    Clockwise,
    AntiClockwise,
}

#[derive(Debug, Reflect, Default, PartialEq, Eq, Clone)]
pub enum CircuitOrientation {
    Horizontal,
    #[default]
    Vertical,
}

fn update_circuit(
    mut gizmos: Gizmos,
    time: Res<Time>,
    mut circuit: ResMut<Circuit>,
    player_query: Query<&Collider, With<Player>>,
    chunks_query: Query<
        (Entity, &Collider, &ChunkRoad, &ChunkConnextions),
        (With<ChunkTag>, Without<Player>),
    >,
) {
    if let Ok(player_collider) = player_query.get_single() {
        for (chunk_entity, chunk_collider, orientation, connextions) in chunks_query.iter() {
            if let Some(collision) = collide(
                player_collider.center().extend(0.),
                player_collider.size(),
                chunk_collider.center().extend(0.),
                chunk_collider.size(),
            ) {
                match &orientation {
                    ChunkRoad::Turn => {
                        #[cfg(feature = "dev")]
                        gizmos.rect_2d(
                            chunk_collider.center(),
                            0.,
                            chunk_collider.size() - 10.,
                            Color::Srgba(BLUE),
                        );

                        if !circuit.turn.contains(&chunk_entity) {
                            let t = match (&collision, &circuit.current_orientation) {
                                (Collision::Bottom, CircuitOrientation::Vertical) => {
                                    println!("1");
                                    let r = if connextions.contains(&ChunkConnextion::Left) {
                                        println!("Turn left");
                                        -1
                                    } else if connextions.contains(&ChunkConnextion::Right) {
                                        println!("Turn right");
                                        1
                                    } else {
                                        0
                                    };
                                    r
                                }
                                (Collision::Left, CircuitOrientation::Horizontal) => {
                                    println!("2");
                                    let r = if connextions.contains(&ChunkConnextion::Top) {
                                        println!("Turn left");
                                        -1
                                    } else if connextions.contains(&ChunkConnextion::Bottom) {
                                        println!("Turn right");
                                        1
                                    } else {
                                        0
                                    };
                                    r
                                }
                                (Collision::Top, CircuitOrientation::Vertical) => {
                                    println!("3");
                                    let r = if connextions.contains(&ChunkConnextion::Left) {
                                        println!("Turn right");
                                        1
                                    } else if connextions.contains(&ChunkConnextion::Right) {
                                        println!("Turn left");
                                        -1
                                    } else {
                                        0
                                    };
                                    r
                                }
                                (Collision::Right, CircuitOrientation::Horizontal) => {
                                    println!("4");
                                    let r = if connextions.contains(&ChunkConnextion::Top) {
                                        println!("Turn right");
                                        1
                                    } else if connextions.contains(&ChunkConnextion::Bottom) {
                                        println!("Turn left");
                                        -1
                                    } else {
                                        0
                                    };
                                    r
                                }

                                _ => 0,
                            };

                            circuit.in_turn = true;

                            circuit.turn_count += t;

                            circuit.current_orientation = match circuit.current_orientation {
                                CircuitOrientation::Horizontal => CircuitOrientation::Vertical,
                                CircuitOrientation::Vertical => CircuitOrientation::Horizontal,
                            };

                            circuit.turn.push(chunk_entity);
                        }
                    }
                    _ => {
                        if !circuit.already_collide.contains(&chunk_entity) {
                            circuit.in_turn = false;
                            circuit.already_collide.push(chunk_entity);
                        }
                    }
                }
            }
        }
    }
}
