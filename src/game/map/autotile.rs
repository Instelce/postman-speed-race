use super::types::MatrixType;

use super::ldtk::AutoLayerRuleDefinition;

/// Represents how a single tile location should be matched when evaluating a rule
#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default, Copy, Clone)]
pub enum TileStatus {
    /// This tile will always match, regardless of the input tile
    #[default]
    Ignore,
    /// This tile will only match when there is no input tile (`None`)
    Nothing,
    /// This tile will always match as long as the tile exists (`Option::is_some`)
    Anything,
    /// This tile will match as long as the input tile exists and the input value is the same as this value
    Is(usize),
    /// This tile will match as long as the input tile exists and the input value is anything other than this value
    IsNot(usize),
}

impl PartialEq<Option<usize>> for TileStatus {
    fn eq(&self, other: &Option<usize>) -> bool {
        match self {
            Self::Ignore => true,
            Self::Nothing => other.is_none(),
            Self::Anything => other.is_some(),
            Self::Is(value) => &Some(*value) == other,
            Self::IsNot(value) => &Some(*value) != other,
        }
    }
}

impl TileStatus {
    pub fn to_ldtk_value(&self) -> i64 {
        match self {
            Self::Ignore => 0,
            Self::Nothing => -1000001,
            Self::Anything => 1000001,
            Self::Is(value) => *value as i64,
            Self::IsNot(value) => -(*value as i64),
        }
    }

    pub fn from_ldtk_value(value: i64) -> Self {
        match value {
            0 => Self::Ignore,
            1000001 => Self::Anything,
            -1000001 => Self::Nothing,
            other => {
                if other > 0 {
                    Self::Is(other as usize)
                } else {
                    Self::IsNot(other.unsigned_abs() as usize)
                }
            }
        }
    }
}

pub enum FlipAxis {
    X, // vertically for LDtk
    Y, // horizontaly
    XY,
}

#[derive(Clone, Debug, Default)]
pub struct TileMatcher {
    pub matcher: Vec<TileStatus>,
    pub size: i64,
}

impl From<&AutoLayerRuleDefinition> for TileMatcher {
    fn from(value: &AutoLayerRuleDefinition) -> Self {
        let mut matrix: Vec<TileStatus> = Vec::new();
        let size = value.size;

        for y in 0..size {
            for x in 0..size {
                let index = y * size + x;
                let tile_status =
                    TileStatus::from_ldtk_value(*value.pattern.get(index as usize).unwrap());
                matrix.push(tile_status);
                // print!("{:?} ", tile_status);
            }
            // println!();
        }

        TileMatcher {
            matcher: matrix,
            size,
        }
    }
}

impl TileMatcher {
    pub fn matches(&self, layout: &TileLayout) -> bool {
        // check if the layout has the same length as the matcher
        assert_eq!(layout.0.len(), self.matcher.len());

        self.matcher
            .iter()
            .zip(layout.0.iter())
            .all(|(status, reality)| {
                // println!(
                //     "{:?} == {:?} ? {} |",
                //     status,
                //     reality,
                //     *status == *reality
                // );
                *status == *reality
            })
    }

    // flip the matcher vertically
    pub fn matches_flip(&self, flip_axis: FlipAxis, layout: &TileLayout) -> bool {
        match flip_axis {
            FlipAxis::X => {
                let flipped_matcher = self.flip_x();
                if flipped_matcher.matches(layout) {
                    return true;
                }
            }
            FlipAxis::Y => {
                if self.flip_y().matches(layout) {
                    return true;
                }
            }
            FlipAxis::XY => {
                let flipped_matcher = self.flip_x().flip_y();
                if flipped_matcher.matches(layout) {
                    return true;
                }
            }
        };

        false
    }

    fn flip_x(&self) -> Self {
        let matrix: MatrixType<TileStatus> = self.to_matrix();
        let mut flipped_matcher: Vec<TileStatus> = Vec::new();

        for row in matrix.iter() {
            for col in row.iter().rev() {
                flipped_matcher.push(col.clone());
            }
        }

        TileMatcher {
            matcher: flipped_matcher,
            size: self.size,
        }
    }

    fn flip_y(&self) -> Self {
        let matrix: MatrixType<TileStatus> = self.to_matrix();
        let mut flipped_matcher: Vec<TileStatus> = Vec::new();

        for row in matrix.iter().rev() {
            for col in row.iter() {
                flipped_matcher.push(col.clone());
            }
        }

        TileMatcher {
            matcher: flipped_matcher,
            size: self.size,
        }
        // TileMatcher { matcher: self.to_matrix().into_iter().rev().flatten().collect(), size: self.size }
    }

    fn to_matrix(&self) -> MatrixType<TileStatus> {
        let mut matrix: MatrixType<TileStatus> = Vec::new();
        for y in 0..self.size {
            let mut row = Vec::new();
            for x in 0..self.size {
                let index = y * self.size + x;
                let tile_status = self.matcher.get(index as usize).unwrap();
                row.push(tile_status.clone());
            }
            matrix.push(row);
        }
        matrix
    }
}

#[derive(Clone, Debug, Default)]
#[repr(transparent)]
pub struct TileLayout(pub Vec<Option<usize>>);

impl TileLayout {
    pub fn from_matrix(matrix: MatrixType<i64>, empty_values: Vec<i64>) -> Self {
        Self(
            matrix
                .into_iter()
                .flatten()
                .map(|v| {
                    if empty_values.contains(&v) {
                        None
                    } else {
                        Some(v as usize)
                    }
                })
                .collect(),
        )
    }
}
