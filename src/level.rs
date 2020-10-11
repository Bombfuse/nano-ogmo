use nanoserde::{DeJson};

use crate::{OgmoError};

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

    pub tileset: String,
    pub data: Vec<i32>,

    #[nserde(rename = "exportMode")]
    pub export_mode: u32,

    #[nserde(rename = "arrayMode")]
    pub array_mode: u32,
}