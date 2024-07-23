use core::num;
use std::default;

use bevy::{
    color::palettes::css::{ORANGE, RED},
    math::VectorSpace,
    prelude::*,
    sprite::Anchor,
};
use bevy_aseprite_ultra::prelude::{Animation, AsepriteAnimationBundle};
use rand::Rng;

use crate::{
    game::{
        assets::handles::{AsepriteAssets, HouseAssets, TilesetAssets},
        collider::{Collider, Collision, ExcludeColliderUpdate},
        house::HouseOrientation,
        letter::{LetterBox, LetterLaunchZone, Letters},
        map::{
            builder::MapBuilder,
            chunk::{ChunkConnextion, ChunkType, RoadChunkType, CHUNK_SIZE, PIXEL_CHUNK_SIZE},
            ldtk::Project,
        },
    },
    utils::get_asset_path,
};

use super::player::SpawnPlayer;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<(MapTag, ChunkTag, ChunkRoad)>();
    app.observe(spawn_map);
}

#[derive(Event, Debug)]
pub struct SpawnMap {
    pub level: i32,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct MapTag;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ChunkTag;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub enum ChunkRoad {
    Horizontal,
    #[default]
    Vertical,
    Turn,
}

#[derive(Component, Default, Deref, DerefMut, Debug)]
pub struct ChunkConnextions(pub Vec<ChunkConnextion>);

fn spawn_map(
    trigger: Trigger<SpawnMap>,
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    tilesets: Res<TilesetAssets>,
    houses: Res<HouseAssets>,
    aseprites: Res<AsepriteAssets>,
) {
    let mut rng = rand::thread_rng();

    // Setup tileset layout
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 25, 25, None, None);
    let texture_atlas_layout = texture_atlases.add(layout);

    // Build the map
    let mut builder = MapBuilder::new(
        Project::new(get_asset_path("maps/maps.ldtk")),
        Project::new(get_asset_path("maps/chunks.ldtk")),
    );

    builder.build(&trigger.event().level);
    let map = builder.get_map();

    // Init letters
    commands.insert_resource(Letters::init(map.count_chunk(ChunkType::House)));

    // Spawn player
    commands.trigger(SpawnPlayer(map.start_position));

    let map_entity = commands
        .spawn((Name::new("Map"), SpatialBundle::default(), MapTag))
        .id();

    // Spawn chunks
    let mut chunks = Vec::new();
    for (i, chunk) in map.chunks.iter().enumerate() {
        if chunk.is_empty() {
            continue;
        }

        // calc rotation and translation of chunk
        let mut angle = 0.;
        let mut translation = chunk.position;

        if chunk.flip_x && chunk.flip_y {
            angle = std::f32::consts::PI;
            translation.x += PIXEL_CHUNK_SIZE - 16.;
            translation.y -= PIXEL_CHUNK_SIZE - 16.;
        } else {
            if chunk.flip_x {
                angle = -std::f32::consts::PI / 2.;
                translation.x += PIXEL_CHUNK_SIZE - 16.;
            }
            if chunk.flip_y {
                angle = std::f32::consts::PI / 2.;
                translation.y -= PIXEL_CHUNK_SIZE - 16.;
            }
        }

        let rotation = Quat::from_axis_angle(Vec3::Z, angle);

        let chunk_entity = commands
            .spawn((
                SpatialBundle {
                    transform: Transform::from_translation(translation.extend(-0.01))
                        .with_rotation(rotation),
                    ..default()
                },
                ChunkConnextions(chunk.connextions.clone()),
                ChunkTag,
            ))
            .with_children(|children| {
                // top left of chunk
                // children.spawn(SpriteBundle {
                //     sprite: Sprite {
                //         color: Color::Srgba(ORANGE),
                //         rect: Some(Rect::from_center_half_size(Vec2::ZERO, Vec2::splat(1.))),
                //         ..default()
                //     },
                //     ..default()
                // });

                // Tiles spawn
                for y in 0..CHUNK_SIZE {
                    for x in 0..CHUNK_SIZE {
                        let intgrid = chunk.intgrid_at(x, y).unwrap();
                        let tile = chunk.tile_at(x, y).unwrap();

                        children.spawn((
                            Name::new("Tile"),
                            SpriteBundle {
                                sprite: Sprite {
                                    flip_x: tile.flip_x,
                                    flip_y: tile.flip_y,
                                    ..default()
                                },
                                texture: tilesets.get("tiles").clone(),
                                transform: Transform::from_translation(Vec3::new(
                                    x as f32 * 16.,
                                    y as f32 * -16.,
                                    -0.01,
                                )),
                                ..default()
                            },
                            TextureAtlas {
                                layout: texture_atlas_layout.clone(),
                                index: tile.value as usize,
                            },
                        ));
                    }
                }
            })
            .id();

        match &chunk.chunk_type {
            ChunkType::Road(road_type) => {
                // get chunk orientation
                let orientation = match road_type {
                    RoadChunkType::Horizontal => ChunkRoad::Horizontal,
                    RoadChunkType::Vertical => ChunkRoad::Vertical,
                    RoadChunkType::Turn => ChunkRoad::Turn,
                };

                commands.entity(chunk_entity).insert((
                    orientation,
                    Collider::new_rect_corners(
                        chunk.position + Vec2::new(-8., 8.),
                        chunk.position
                            + Vec2::new(PIXEL_CHUNK_SIZE, -PIXEL_CHUNK_SIZE)
                            + Vec2::new(-8., 8.),
                    ),
                ));
            }
            ChunkType::House => {
                // House spawn
                let house = &chunk.house.clone().unwrap();
                let number = rng.gen_range(1..=6);
                let house_postition = (house.position * Vec2::new(1., -1.));
                let house_entity = commands
                    .spawn((
                        Name::new("House"),
                        SpriteBundle {
                            sprite: Sprite {
                                anchor: Anchor::BottomRight,
                                ..default()
                            },
                            texture: houses.get(number.to_string().as_str()),
                            transform: Transform::from_translation(house_postition.extend(0.001)),
                            ..default()
                        },
                    ))
                    .id();

                // Spawn letter box
                let letter_box_position = house_postition + Vec2::new(16. * 2., 16.);
                let letter_box_entity = commands
                    .spawn((
                        Name::new("Letter Box"),
                        AsepriteAnimationBundle {
                            aseprite: aseprites.get("letter-box"),
                            animation: Animation::default().with_tag("close"),
                            transform: Transform::from_translation(letter_box_position.extend(0.)),
                            ..default()
                        },
                        Collider::rect(8., 8.),
                        LetterBox,
                    ))
                    .id();

                commands
                    .entity(chunk_entity)
                    .push_children(&[house_entity, letter_box_entity]);

                // Spawn letter launch zone
                let collider_for_letter_boooox;
                let collider_size = Vec2::new(
                    PIXEL_CHUNK_SIZE / 2.,
                    PIXEL_CHUNK_SIZE / 2. + PIXEL_CHUNK_SIZE / 4.,
                );
                let angle_mul = if chunk.has_connexion(ChunkConnextion::Right) {
                    collider_for_letter_boooox = commands
                        .spawn((Collider::new_rect(
                            chunk.position + (Vec2::X * (PIXEL_CHUNK_SIZE + PIXEL_CHUNK_SIZE / 2.)),
                            collider_size,
                        ),))
                        .id();
                    0
                } else if chunk.has_connexion(ChunkConnextion::Bottom) {
                    collider_for_letter_boooox = commands
                        .spawn((Collider::new_rect(
                            chunk.position - (Vec2::Y * (PIXEL_CHUNK_SIZE + PIXEL_CHUNK_SIZE / 2.)),
                            Vec2::new(collider_size.y, collider_size.x),
                        ),))
                        .id();
                    1
                } else if chunk.has_connexion(ChunkConnextion::Left) {
                    collider_for_letter_boooox = commands
                        .spawn((Collider::new_rect(
                            chunk.position - Vec2::X * PIXEL_CHUNK_SIZE / 2.,
                            collider_size,
                        ),))
                        .id();
                    2
                } else if chunk.has_connexion(ChunkConnextion::Top) {
                    collider_for_letter_boooox = commands
                        .spawn((Collider::new_rect(
                            chunk.position + Vec2::Y * PIXEL_CHUNK_SIZE / 2.,
                            Vec2::new(collider_size.y, collider_size.x),
                        ),))
                        .id();
                    3
                } else {
                    collider_for_letter_boooox = commands
                        .spawn((Collider::new_rect(Vec2::ZERO, collider_size),))
                        .id();
                    0
                };
                commands.entity(collider_for_letter_boooox).insert((
                    Transform::default(),
                    ExcludeColliderUpdate,
                    LetterLaunchZone(letter_box_entity),
                ));
                commands
                    .entity(chunk_entity)
                    .insert(HouseOrientation { angle_mul })
                    .push_children(&[collider_for_letter_boooox]);
            }
            _ => {}
        }

        commands
            .entity(chunk_entity)
            .insert(Name::new(format!("Chunk {:?}", chunk.chunk_type)));

        chunks.push(chunk_entity);
    }

    commands.entity(map_entity).push_children(chunks.as_slice());
}
