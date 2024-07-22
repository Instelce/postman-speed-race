use super::types::MatrixType;
use image::{imageops, ImageBuffer, Rgba};

use super::transformer::Matrix;

/// convert inline csv to a Vec matrix
pub fn inline_csv_to_matrix(csv: Vec<i64>, height: i64, width: i64) -> MatrixType<i64> {
    let mut matrix = Vec::new();
    for y in 0..height {
        let mut col = Vec::new();
        for x in 0..width {
            let index = y * height + x;
            let value = csv.get(index as usize).unwrap();
            col.push(value.clone());
        }
        matrix.push(col);
    }
    matrix
}

/// Optimize a map by removing useless tiles
pub fn optimize_map(matrix: MatrixType<i64>) -> MatrixType<i64> {
    let map = Matrix::new(matrix.clone());
    let mut result = matrix.clone();
    for (y, row) in map.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            if *value == 2 {
                let surrouding_tiles =
                    map.get_surrounding_tiles(x as i64, y as i64, 1, value.clone().into());

                if surrouding_tiles.iter().all(|r| r.iter().all(|v| *v == 2)) {
                    result[y][x] = -1;
                }
            }
        }
    }
    result
}

/// Cut a tileset into tiles of size `tile_size`.
/// `path` is the path to the tileset image
pub fn cut_tileset(
    path: String,
    width: i64,
    height: i64,
    tile_size: u32,
) -> Vec<ImageBuffer<Rgba<u8>, Vec<u8>>> {
    let mut tiles = Vec::new();
    let mut tileset_image = image::open(path).expect("Cannot open image");

    for row in 0..height as u32 {
        for col in 0..width as u32 {
            let x = col * tile_size;
            let y = row * tile_size;

            // crop tileset to get the tile at (x, y) coords
            let tile_image =
                imageops::crop(&mut tileset_image, x, y, tile_size, tile_size).to_image();

            tiles.push(tile_image);
        }
    }

    tiles
}
