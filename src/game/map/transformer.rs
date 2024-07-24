use std::{fmt::Display, ops::Deref};

use bevy::prelude::*;

use super::types::{MatrixType, Tile};

use super::{
    autotile::{FlipAxis, TileLayout, TileMatcher},
    ldtk::{Project, Type},
};

#[derive(Clone)]
pub struct Matrix<T: Clone + Display>(pub MatrixType<T>);

impl<T: Clone + Display> Matrix<T> {
    pub fn empty() -> Self {
        Self(Vec::new())
    }

    pub fn new(matrix: MatrixType<T>) -> Self {
        Self(matrix)
    }

    pub fn from_size(width: i64, height: i64, value: T) -> Self {
        let mut matrix = Vec::new();
        for _ in 0..height {
            let mut row = Vec::new();
            for _ in 0..width {
                row.push(value.clone());
            }
            matrix.push(row);
        }
        Self(matrix)
    }

    /// get the surrounding tiles of a tile at (x, y) coords with a scope of `scope`
    pub fn get_surrounding_tiles(
        &self,
        x: i64,
        y: i64,
        scope: i64,
        default_value: T,
    ) -> MatrixType<T> {
        let mut matrix = Vec::new();
        for i in -scope..=scope {
            let mut row = Vec::new();
            for j in -scope..=scope {
                let dy = i + y;
                let dx = j + x;

                match self.0.get(dy as usize) {
                    Some(col) => match col.get(dx as usize) {
                        Some(tile) => {
                            row.push(tile.clone());
                        }
                        None => {
                            row.push(default_value.clone());
                        } // ignore value
                    },
                    None => {
                        row.push(default_value.clone());
                    }
                }
            }
            matrix.push(row);
        }

        matrix
    }

    fn height(&self) -> usize {
        self.0.len()
    }

    fn width(&self) -> usize {
        self.0[0].len()
    }
}

impl<T: Display + Clone> Deref for Matrix<T> {
    type Target = MatrixType<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Clone + Display> Display for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.0 {
            for val in row {
                let width = 4;
                write!(f, "{val:>width$},")?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

/// Generate a level from a csv map with LDtk rules (IntGrid and AutoLayer)
///
/// Args
/// ---
/// - `csv`: a Vec of Vec of i64, the csv map
/// - `project`: a reference to the LDtk project, use to get the rules
pub fn generate_level(
    matrix: MatrixType<i64>,
    project: &Project,
    layer_index: Option<usize>,
    hide_layers: Option<Vec<usize>>,
) -> MatrixType<Tile> {
    let map_height = matrix.len() as u32;
    let map_width = matrix.get(0).unwrap().len() as u32;
    let map = Matrix::new(matrix.clone());

    let mut layers: Vec<MatrixType<Tile>> = Vec::new();

    for (id, layer) in project.defs.layers.iter().enumerate() {
        if let Some(layer_ids) = &hide_layers {
            if layer_ids.contains(&id) {
                continue;
            }
        }

        match layer.purple_type {
            // generation of IntGrid layers
            Type::IntGrid => {
                let mut gen_map: MatrixType<Tile> = Vec::new();

                for (y, row) in map.iter().enumerate() {
                    let mut gen_row = Vec::new();
                    for (x, value) in row.iter().enumerate() {
                        // check if the current value is an IntGrid value
                        match layer.get_intgrid_value_definition(value) {
                            Some(intgrid_value_def) => {
                                // we only test rules that are in the group
                                // with the same name as the value (IntGrid),
                                // so we avoid testing useless rules

                                if let None =
                                    intgrid_value_def.get_auto_rule_group(&layer.auto_rule_groups)
                                {
                                    continue;
                                }

                                let auto_rule_group = intgrid_value_def
                                    .get_auto_rule_group(&layer.auto_rule_groups)
                                    .expect("Group not found");

                                // loop over rules
                                let mut find_rule = false;
                                for rule in auto_rule_group.rules.iter() {
                                    let scope = rule.size / 2;

                                    // get the surrounding tiles of the current tile
                                    let surrounding_tiles = map.get_surrounding_tiles(
                                        x as i64,
                                        y as i64,
                                        scope,
                                        intgrid_value_def.value,
                                    );

                                    let layout =
                                        TileLayout::from_matrix(surrounding_tiles, vec![0, -1]);
                                    let matcher = TileMatcher::from(rule);

                                    // check if the rule matches
                                    if matcher.matches(&layout) {
                                        gen_row.push(Tile::new(rule.get_single_tile_id()));
                                        find_rule = true;
                                        break;
                                    }

                                    // check if the flipped x rule matches
                                    if rule.flip_x {
                                        if matcher.matches_flip(FlipAxis::X, &layout) {
                                            gen_row.push(Tile {
                                                value: rule.get_single_tile_id(),
                                                flip_x: true,
                                                ..default()
                                            });
                                            find_rule = true;
                                            break;
                                        }
                                    }
                                }

                                if !find_rule {
                                    gen_row.push(Tile::default());
                                }
                            }
                            None => {
                                gen_row.push(Tile::new(value.clone()));
                            }
                        }
                    }
                    gen_map.push(gen_row);
                }

                layers.push(gen_map);
            }

            // generation of AutoLayers
            Type::AutoLayer => {
                let mut gen_map = Vec::new();
                let intgrid_values = project.all_intgrid_values();

                for (y, row) in map.iter().enumerate() {
                    let mut gen_row = Vec::new();
                    for (x, value) in row.iter().enumerate() {
                        let mut find_rule = false;
                        for rule in layer.all_rules() {
                            let scope = rule.size / 2;

                            // get the surrounding tiles of the current tile
                            let surrounding_tiles =
                                map.get_surrounding_tiles(x as i64, y as i64, scope, value.clone());

                            let layout = TileLayout::from_matrix(surrounding_tiles, vec![0, -1]);
                            let matcher = TileMatcher::from(rule);

                            // check if the rule matches
                            if matcher.matches(&layout) {
                                gen_row.push(Tile::new(rule.get_single_tile_id()));
                                find_rule = true;
                                break;
                            }

                            // check if the flipped x rule matches
                            if rule.flip_x {
                                if matcher.matches_flip(FlipAxis::X, &layout) {
                                    gen_row.push(Tile {
                                        value: rule.get_single_tile_id(),
                                        flip_x: true,
                                        ..default()
                                    });
                                    find_rule = true;
                                    break;
                                }
                            }

                            if rule.flip_y {
                                if matcher.matches_flip(FlipAxis::Y, &layout) {
                                    gen_row.push(Tile {
                                        value: rule.get_single_tile_id(),
                                        flip_y: true,
                                        ..default()
                                    });
                                    find_rule = true;
                                    break;
                                }
                            }

                            if rule.flip_x && rule.flip_y {
                                if matcher.matches_flip(FlipAxis::XY, &layout) {
                                    gen_row.push(Tile {
                                        value: rule.get_single_tile_id(),
                                        flip_x: true,
                                        flip_y: true,
                                    });
                                    find_rule = true;
                                    break;
                                }
                            }
                        }

                        if !find_rule {
                            gen_row.push(Tile::default());
                        }
                    }
                    gen_map.push(gen_row);
                }

                layers.push(gen_map.clone());
            }

            _ => {}
        }
    }

    // superposition of layers
    // generation of the final grid
    let mut final_map = Matrix::from_size(map_width.into(), map_height.into(), Tile::default());

    if let Some(indice) = layer_index {
        let matrix = &layers[indice];
        for y in 0..map_height {
            for x in 0..map_width {
                let tile = &matrix[y as usize][x as usize];
                if tile.value != 0 {
                    final_map.0[y as usize][x as usize] = tile.clone();
                }
            }
        }
    } else {
        // reverse layers because LDtk layers order is reversed
        for matrix in layers.iter().rev() {
            for y in 0..map_height {
                for x in 0..map_width {
                    let tile = &matrix[y as usize][x as usize];
                    if tile.value != 0 {
                        final_map.0[y as usize][x as usize] = tile.clone();
                    }
                }
            }
        }
    }

    final_map.0
}
