use std::fmt::Display;

use serde::{Deserialize, Serialize};

pub type MatrixType<T> = Vec<Vec<T>>;

#[derive(PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum IntgridType {
    Road,
    Dirt,
}

impl IntgridType {
    pub fn to_i64(&self) -> i64 {
        match self {
            Self::Road => 1,
            Self::Dirt => 2,
        }
    }
}

impl From<&i64> for IntgridType {
    fn from(value: &i64) -> Self {
        match value {
            1 => Self::Road,
            2 => Self::Dirt,
            _ => Self::Dirt,
        }
    }
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct Tile {
    pub value: i64,
    pub flip_x: bool,
    pub flip_y: bool,
}

impl Tile {
    pub fn new(value: i64) -> Self {
        Self {
            value,
            ..Default::default()
        }
    }

    pub fn to_i64(&self) -> i64 {
        self.value
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
