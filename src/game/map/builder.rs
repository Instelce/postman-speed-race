use std::collections::HashMap;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::{
    chunk::{Chunk, ChunkType, CHUNK_SIZE, PIXEL_CHUNK_SIZE},
    ldtk::Project,
    transformer::generate_level,
    types::IntgridType,
    utils::inline_csv_to_matrix,
};

pub struct MapBuilder {
    maps: Project,
    chunks: HashMap<ChunkType, Chunk>,
    map: Map,
}

impl MapBuilder {
    pub fn new(maps: Project, chunks_project: Project) -> Self {
        // retrieve all chunks data
        // level of chunks_project = chunk
        let mut chunks = HashMap::new();
        for level in &chunks_project.levels {
            let csv = level.get_layer("Intgrid").int_grid_csv.clone();
            let height = level.px_hei / 16;
            let width = level.px_wid / 16;

            // Intgrid tiles
            let mut intgrid_tiles = Vec::new();
            for value in &csv {
                intgrid_tiles.push(IntgridType::from(value));
            }

            // Tiles
            let tiles = generate_level(
                inline_csv_to_matrix(csv, height, width),
                &chunks_project,
                None,
                None,
            );

            let chunk_type = ChunkType::from(level.identifier.as_str());

            chunks.insert(
                chunk_type.clone(),
                Chunk {
                    intgrid_tiles,
                    tileset_tiles: tiles.into_iter().flatten().collect(),
                    chunk_type,
                    ..default()
                },
            );
        }

        Self {
            maps,
            chunks,
            map: Map::default(),
        }
    }

    /// Build the map
    pub fn build(&mut self, level_indice: &i32) {
        // récupérer le "base" layer
        // pour chaque intgrid ajouter le chunk a la position
        let map = self.maps.levels.get(*level_indice as usize).unwrap();
        let base = map.get_layer("Base").int_grid_csv.clone();

        // Tiles
        let tiles = generate_level(
            inline_csv_to_matrix(base.clone(), map.tile_y(), map.tile_x()),
            &self.maps,
            Some(1),
            None,
        );

        for (y, row) in tiles.iter().enumerate() {
            for (x, chunk_tile) in row.iter().enumerate() {
                let intgrid_value = base[y * map.tile_y() as usize + x];
                // let chunk_type = MapIntgrid::from(&intgrid_value);

                if chunk_tile.value != 0 {
                    let mut chunk = self
                        .chunks
                        .get(&ChunkType::from(&chunk_tile.value))
                        .unwrap()
                        .clone();
                    chunk.position = Vec2::new(
                        PIXEL_CHUNK_SIZE as f32 * x as f32,
                        -PIXEL_CHUNK_SIZE as f32 * y as f32,
                    );

                    if chunk_tile.value == 2 {
                        chunk.flip_x = true;
                    }
                    if chunk_tile.value == 6 {
                        chunk.flip_y = true;
                    }
                    if chunk_tile.value == 7 {
                        chunk.flip_x = true;
                        chunk.flip_y = true;
                    }

                    self.map.chunks.push(chunk);
                }
            }
        }
    }

    pub fn get_map(&self) -> Map {
        self.map.clone()
    }
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Map {
    pub chunk_x: i32,
    pub chunk_y: i32,
    pub chunks: Vec<Chunk>,
    // interactables: Vec<Interactable>,
    // houses: Vec<House>,
}

#[derive(PartialEq, Eq)]
pub enum MapIntgrid {
    Road,
    House,
    PostOffice,
}

impl From<&i64> for MapIntgrid {
    fn from(value: &i64) -> Self {
        match value {
            1 => Self::Road,
            2 => Self::House,
            3 => Self::PostOffice,
            _ => Self::House,
        }
    }
}
