use std::default;

use bevy::{color::palettes::css::ORANGE, prelude::*};

use crate::{
    game::{
        assets::handles::TilesetAssets,
        collider::{Collider, Collision},
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
    Vertical,
    Turn,
    #[default]
    Not,
}

#[derive(Component, Default, Deref, DerefMut, Debug)]
pub struct ChunkConnextions(pub Vec<ChunkConnextion>);

fn spawn_map(
    trigger: Trigger<SpawnMap>,
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    tilesets: Res<TilesetAssets>,
) {
    // Setup tileset layout
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 25, 25, None, None);
    let texture_atlas_layout = texture_atlases.add(layout);

    // build the map
    let mut builder = MapBuilder::new(
        Project::new(get_asset_path("maps/maps.ldtk")),
        Project::new(get_asset_path("maps/chunks.ldtk")),
    );

    builder.build(&trigger.event().level);
    let map = builder.get_map();

    commands.trigger(SpawnPlayer(map.start_position));

    commands
        .spawn((Name::new("Map"), SpatialBundle::default(), MapTag))
        .with_children(|children| {
            // spawn chunks
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

                // get chunk orientation
                let mut orientation = ChunkRoad::Not;

                match &chunk.chunk_type {
                    ChunkType::Road(road_type) => {
                        orientation = match road_type {
                            RoadChunkType::Horizontal => ChunkRoad::Horizontal,
                            RoadChunkType::Vertical => ChunkRoad::Vertical,
                            RoadChunkType::Turn => ChunkRoad::Turn,
                        };
                    }
                    _ => {}
                }

                children
                    .spawn((
                        Name::new("Chunk"),
                        SpatialBundle {
                            transform: Transform::from_translation(translation.extend(0.))
                                .with_rotation(rotation),
                            ..default()
                        },
                        Collider::new_rect_corners(
                            chunk.position,
                            chunk.position
                                + Vec2::new(PIXEL_CHUNK_SIZE - 16., -PIXEL_CHUNK_SIZE + 16.),
                        ),
                        ChunkConnextions(chunk.connextions.clone()),
                        orientation,
                        ChunkTag,
                    ))
                    .with_children(|children| {
                        // top left of chunk
                        children.spawn(SpriteBundle {
                            sprite: Sprite {
                                color: Color::Srgba(ORANGE),
                                rect: Some(Rect::from_center_half_size(
                                    Vec2::new(PIXEL_CHUNK_SIZE / 2., PIXEL_CHUNK_SIZE / 2.),
                                    Vec2::splat(1.),
                                )),
                                ..default()
                            },
                            ..default()
                        });

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
                    });
            }
        });
}
