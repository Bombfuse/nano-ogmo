use nanoserde::{DeJson};

use crate::{OgmoError};

use std::collections::HashMap;

#[derive(Clone, Debug, DeJson)]
pub struct Level {
    #[nserde(rename = "ogmoVersion")]
    pub ogmo_version: String,

    pub width: u32,
    pub height: u32,

    #[nserde(rename = "offsetX")]
    pub offset_x: u32,

    #[nserde(rename = "offsetY")]
    pub offset_y: u32,

    pub layers: Vec<LevelLayer>,
}
impl Level {
    pub fn from_json(json: &String) -> Result<Level, OgmoError> {
        let level: Level = DeJson::deserialize_json(json)?;

        Ok(level)
    }
}

#[derive(Clone, Debug, DeJson)]
pub struct LevelLayer {
    pub name: String,

    pub entities: Option<Vec<LevelEntity>>,
    pub tileset: Option<String>,
    pub data: Option<Vec<i32>>,

    #[nserde(rename = "_eid")]
    pub export_id: String,

    #[nserde(rename = "offsetX")]
    pub offset_x: u32,

    #[nserde(rename = "offsetY")]
    pub offset_y: u32,

    #[nserde(rename = "gridCellWidth")]
    pub grid_cell_width: u32,

    #[nserde(rename = "gridCellHeight")]
    pub grid_cell_height: u32,

    #[nserde(rename = "exportMode")]
    pub export_mode: u32,

    #[nserde(rename = "arrayMode")]
    pub array_mode: u32,
}

#[derive(Clone, Debug, DeJson)]
pub struct LevelEntity {
    pub name: String,
    pub id: u32,

    #[nserde(rename = "_eid")]
    pub export_id: String,

    pub x: i32,
    pub y: i32,

    #[nserde(rename = "originX")]
    pub origin_x: i32,

    #[nserde(rename = "originY")]
    pub origin_y: i32,

    pub values: Option<HashMap<String, String>>,
}