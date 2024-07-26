mod json_1_5_3;
use std::{fs::File, io::BufReader, path::Path};

use bevy::asset::embedded_asset;
pub use json_1_5_3::*;

impl Project {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let file = BufReader::new(File::open(&path).unwrap());
        let project: Project = serde_json::from_reader(file).unwrap();
        project
    }

    pub fn count_level(&self) -> i32 {
        self.levels.iter().count() as i32
    }

    pub fn all_intgrid_values(&self) -> Vec<&IntGridValueDefinition> {
        let mut values: Vec<&IntGridValueDefinition> = Vec::new();
        for layer in self.defs.layers.iter() {
            for value in layer.int_grid_values.iter() {
                values.push(value);
            }
        }
        values
    }
}

impl Level {
    pub fn get_field(&self, field_name: &str) -> &FieldInstance {
        self.field_instances
            .iter()
            .find(|field| field.identifier == field_name)
            .unwrap()
    }

    pub fn get_layer(&self, layer_name: &str) -> &LayerInstance {
        self.layer_instances
            .as_ref()
            .unwrap()
            .iter()
            .find(|layer| layer.identifier == layer_name)
            .unwrap()
    }

    #[inline]
    pub fn tile_x(&self) -> i64 {
        self.px_wid / 16
    }

    #[inline]
    pub fn tile_y(&self) -> i64 {
        self.px_hei / 16
    }
}

impl LayerDefinition {
    pub fn get_intgrid_value_definition(&self, value: &i64) -> Option<&IntGridValueDefinition> {
        match self.int_grid_values.iter().position(|v| v.value == *value) {
            Some(indice) => Some(&self.int_grid_values.get(indice).unwrap()),
            None => None,
        }
    }

    pub fn get_intgrid_value_definition_position(&self, value: &i64) -> Option<usize> {
        self.int_grid_values.iter().position(|v| v.value == *value)
    }

    pub fn all_rules(&self) -> Vec<&AutoLayerRuleDefinition> {
        let mut rules_groups = Vec::new();
        for group in &self.auto_rule_groups {
            rules_groups.push(&group.rules);
        }
        rules_groups.into_iter().flatten().collect()
    }
}

impl IntGridValueDefinition {
    pub fn get_auto_rule_group<'a>(
        &'a self,
        groups: &'a Vec<AutoLayerRuleGroup>,
    ) -> Option<&AutoLayerRuleGroup> {
        match groups
            .iter()
            .position(|group| group.name == self.identifier.clone().unwrap())
        {
            Some(indice) => Some(groups.get(indice).unwrap()),
            None => None,
        }
    }
}

impl AutoLayerRuleDefinition {
    pub fn get_single_tile_id(&self) -> i64 {
        if self.tile_rects_ids.len() == 1 {
            self.tile_rects_ids.get(0).unwrap().get(0).unwrap().clone()
        } else {
            0
        }
    }
}
