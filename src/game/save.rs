use crate::utils::{get_asset_path, path_exist};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::{read_to_string, File};
use std::io::Write;

use super::map::ldtk::Project;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<GameSave>();
}

#[derive(Resource, Reflect, Serialize, Deserialize, Default, Debug)]
#[reflect(Resource)]
pub struct GameSave {
    pub last_level_passed: i32,
    pub levels: Vec<LevelData>,
}

impl GameSave {
    pub fn load() -> Self {
        let mut game = GameSave::default();
        let path = &get_asset_path("data/save.ron");

        if path_exist(path) {
            let file = read_to_string(path).unwrap();
            game = ron::from_str(&file).unwrap();
        } else {
            // create the file if it doesn't exist
            let mut file = File::create("assets/data/save.ron").unwrap();
            file.write_all(&ron::to_string(&game).unwrap().as_bytes())
                .unwrap();
        }

        let maps = Project::new(get_asset_path("maps/maps.ldtk"));
        for level in &maps.levels {
            let name = level
                .get_field("Name")
                .value
                .as_ref()
                .unwrap()
                .to_string()
                .clone()
                .replace("\"", "");
            game.levels.push(LevelData { name });
        }

        game
    }
}

#[derive(Resource, Reflect, Serialize, Deserialize, Default, Debug)]
pub struct LevelData {
    pub name: String,
}
