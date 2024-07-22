use bevy::prelude::*;

use crate::{
    game::{
        assets::handles::TilesetAssets,
        map::{
            builder::MapBuilder,
            chunk::{CHUNK_SIZE, PIXEL_CHUNK_SIZE},
            ldtk::Project,
        },
    },
    utils::get_asset_path,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<ChunkTag>();
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

fn spawn_map(
    trigger: Trigger<SpawnMap>,
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    tilesets: Res<TilesetAssets>,
) {
    // Setup tileset layout
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 25, 25, None, None);
    let texture_atlas_layout = texture_atlases.add(layout);

    commands
        .spawn((Name::new("Map"), SpatialBundle::default(), MapTag))
        .with_children(|children| {
            // build the map
            let mut builder = MapBuilder::new(
                Project::new(get_asset_path("maps/maps.ldtk")),
                Project::new(get_asset_path("maps/chunks.ldtk")),
            );

            builder.build(&trigger.event().level);
            let map = builder.get_map();

            // spawn chunks
            for chunk in map.chunks {
                let mut angle = 0.;
                let mut transform = chunk.position;

                if chunk.flip_x && chunk.flip_y {
                    angle = std::f32::consts::PI;
                    transform.x += PIXEL_CHUNK_SIZE as f32 - 16.;
                    transform.y -= PIXEL_CHUNK_SIZE as f32 - 16.;
                } else {
                    if chunk.flip_x {
                        angle = -std::f32::consts::PI / 2.;
                        transform.x += PIXEL_CHUNK_SIZE as f32 - 16.;
                    }
                    if chunk.flip_y {
                        angle = std::f32::consts::PI / 2.;
                        transform.y -= PIXEL_CHUNK_SIZE as f32 - 16.;
                    }
                }

                let rotation = Quat::from_axis_angle(Vec3::Z, angle);

                children
                    .spawn((
                        Name::new("Chunk"),
                        SpatialBundle {
                            transform: Transform::from_translation(transform.extend(0.))
                                .with_rotation(rotation),
                            ..default()
                        },
                        ChunkTag,
                    ))
                    .with_children(|children| {
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
