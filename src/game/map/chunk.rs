use std::default;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::types::{IntgridType, Tile};

pub const CHUNK_SIZE: i32 = 9;
pub const PIXEL_CHUNK_SIZE: f32 = CHUNK_SIZE as f32 * 16.;

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct Chunk {
    pub intgrid_tiles: Vec<IntgridType>,
    pub tileset_tiles: Vec<Tile>,
    pub position: Vec2,
    pub chunk_type: ChunkType,
    pub connextions: Vec<ChunkConnextion>,
    pub house: Option<House>,
    pub is_end: bool,
    pub flip_x: bool,
    pub flip_y: bool,
}

impl Chunk {
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        ((y as u64 * CHUNK_SIZE as u64) + (x as u64)) as usize
    }

    pub fn intgrid_at(&self, x: i32, y: i32) -> Option<&IntgridType> {
        self.intgrid_tiles.get(self.xy_idx(x, y))
    }

    pub fn tile_at(&self, x: i32, y: i32) -> Option<&Tile> {
        self.tileset_tiles.get(self.xy_idx(x, y))
    }

    pub fn is_empty(&self) -> bool {
        self.chunk_type == ChunkType::Empty
    }

    pub fn has_connexion(&self, connexion: ChunkConnextion) -> bool {
        self.connextions.contains(&connexion)
    }
}

#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ChunkType {
    Road(RoadChunkType),
    House,
    PostOffice,
    #[default]
    Empty,
}

impl From<&str> for ChunkType {
    fn from(value: &str) -> Self {
        match value {
            "Vertical" => ChunkType::Road(RoadChunkType::Vertical),
            "VerticalJLeft" => ChunkType::Road(RoadChunkType::VerticalJLeft),
            "VerticalJRight" => ChunkType::Road(RoadChunkType::VerticalJRight),
            "Horizontal" => ChunkType::Road(RoadChunkType::Horizontal),
            "HorizontalJUp" => ChunkType::Road(RoadChunkType::HorizontalJUp),
            "HorizontalJDown" => ChunkType::Road(RoadChunkType::HorizontalJDown),
            "Turn" => ChunkType::Road(RoadChunkType::Turn),
            "House" => ChunkType::House,
            "PostOffice" => ChunkType::PostOffice,
            _ => ChunkType::House,
        }
    }
}

impl From<&i64> for ChunkType {
    fn from(value: &i64) -> Self {
        match value {
            3 => ChunkType::Road(RoadChunkType::Vertical),
            4 => ChunkType::Road(RoadChunkType::Horizontal),
            1 => ChunkType::Road(RoadChunkType::Turn),
            2 => ChunkType::Road(RoadChunkType::Turn),
            6 => ChunkType::Road(RoadChunkType::Turn),
            7 => ChunkType::Road(RoadChunkType::Turn),
            8 => ChunkType::Road(RoadChunkType::VerticalJLeft),
            9 => ChunkType::Road(RoadChunkType::HorizontalJUp),
            13 => ChunkType::Road(RoadChunkType::VerticalJRight),
            14 => ChunkType::Road(RoadChunkType::HorizontalJDown),
            15 => ChunkType::PostOffice,
            16 => ChunkType::House,
            _ => ChunkType::House,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)]
pub enum RoadChunkType {
    Horizontal,
    HorizontalJUp,
    HorizontalJDown,
    Vertical,
    VerticalJLeft,
    VerticalJRight,
    Turn,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ChunkConnextion {
    Top,
    Right,
    Bottom,
    Left,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct House {
    pub position: Vec2,
    pub rotation: f32,
}
