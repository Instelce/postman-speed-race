use std::collections::HashMap;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::{
    chunk::{self, Chunk, ChunkConnextion, ChunkType, RoadChunkType, CHUNK_SIZE, PIXEL_CHUNK_SIZE},
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
        let mut width = 1;
        let mut height = 1;
        for level in &chunks_project.levels {
            let csv = level.get_layer("Intgrid").int_grid_csv.clone();
            height = level.px_hei / 16;
            width = level.px_wid / 16;

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
            map: Map {
                chunk_x: width as i32,
                chunk_y: height as i32,
                ..default()
            },
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
                let mut chunk = Chunk::default(); // empty chunk

                if chunk_tile.value != 0 {
                    let chunk_type = ChunkType::from(&chunk_tile.value);
                    chunk = self.chunks.get(&chunk_type).unwrap().clone();
                    chunk.position =
                        Vec2::new(PIXEL_CHUNK_SIZE * x as f32, -PIXEL_CHUNK_SIZE * y as f32);

                    // add connextions
                    let mut connexions = Vec::new();
                    if tiles[y - 1][x].value != 0 {
                        connexions.push(ChunkConnextion::Top);
                    }
                    if tiles[y + 1][x].value != 0 {
                        connexions.push(ChunkConnextion::Bottom);
                    }
                    if tiles[y][x + 1].value != 0 {
                        connexions.push(ChunkConnextion::Right);
                    }
                    if tiles[y][x - 1].value != 0 {
                        connexions.push(ChunkConnextion::Left);
                    }

                    println!("{:?}, {:?}", chunk_type, connexions);
                    chunk.connextions = connexions;

                    // set start position
                    if chunk_type == ChunkType::PostOffice {
                        self.map.start_position = Vec2::new(
                            chunk.position.x + PIXEL_CHUNK_SIZE + PIXEL_CHUNK_SIZE / 2. - 8.,
                            chunk.position.y - PIXEL_CHUNK_SIZE / 2. + 8.,
                        )
                    }

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
                }

                self.map.chunks.push(chunk);
            }
        }

        println!("{}, {}", self.map.chunk_x, self.map.chunk_y);
        println!("{:?}", self.map.not_empty_chunks());
    }

    pub fn get_map(&self) -> Map {
        self.map.clone()
    }
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct Map {
    pub chunk_x: i32,
    pub chunk_y: i32,
    pub chunks: Vec<Chunk>,
    pub start_position: Vec2,
    // interactables: Vec<Interactable>,
    // houses: Vec<House>,
}

impl Map {
    pub fn get_chunk(&self, x: i32, y: i32) -> &Chunk {
        &self.chunks[(y * self.chunk_y + x) as usize]
    }

    /// Return connexions of a chunk
    pub fn get_connections(&self, x: i32, y: i32) -> Vec<ChunkConnextion> {
        let mut connexions = Vec::new();

        if !self.get_chunk(x, y + 1).is_empty() {
            connexions.push(ChunkConnextion::Top);
        }
        if !self.get_chunk(x, y - 1).is_empty() {
            connexions.push(ChunkConnextion::Bottom);
        }
        if !self.get_chunk(x + 1, y).is_empty() {
            connexions.push(ChunkConnextion::Right);
        }
        if !self.get_chunk(x - 1, y).is_empty() {
            connexions.push(ChunkConnextion::Left);
        }

        if self.get_chunk(x, y).chunk_type == ChunkType::Road(RoadChunkType::Turn) {
            print!("- {:?}", connexions);
        }

        connexions.clone()
    }

    pub fn not_empty_chunks(&self) -> usize {
        self.chunks.iter().filter(|chunk| !chunk.is_empty()).count()
    }
}

pub struct House {
    number: i32,
    position: Vec2,
    flip_x: bool,
    flip_y: bool,
}
