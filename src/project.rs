use nanoserde::{DeJson};

use crate::{Vec2i32, OgmoError};

#[derive(Clone, Debug, DeJson)]
pub struct Project {
    pub name: String,

    #[nserde(rename = "ogmoVersion")]
    pub ogmo_version: String,

    #[nserde(default)]
    #[nserde(rename = "levelPaths")]
    pub level_paths: Vec<String>,

    #[nserde(rename = "backgroundColor")]
    pub background_color: String,

    #[nserde(rename = "gridColor")]
    pub grid_color: String,

    #[nserde(rename = "anglesRadians")]
    pub is_angles_radians: bool,

    #[nserde(rename = "directoryDepth")]
    pub directory_depth: u32,

    #[nserde(rename = "layerGridDefaultSize")]
    pub layer_grid_default_size: Vec2i32,

    #[nserde(rename = "levelDefaultSize")]
    pub level_default_size: Vec2i32,

    #[nserde(rename = "levelMinSize")]
    pub level_min_size: Vec2i32,

    #[nserde(rename = "levelMaxSize")]
    pub level_max_size: Vec2i32,

    pub layers: Vec<Layer>,

    pub entities: Vec<Entity>,
}

impl Project {
    pub fn from_json(json: &String) -> Result<Project, OgmoError> {
        let proj: Project = DeJson::deserialize_json(json)?;

        Ok(proj)
    }
}


#[derive(Clone, Debug, DeJson)]
pub struct Layer {
    pub definition: String,
    pub name: String,

    #[nserde(rename = "gridSize")]
    pub grid_size: Vec2i32,

    #[nserde(rename = "exportID")]
    pub export_id: String,

    #[nserde(rename = "exportMode")]
    pub export_mode: u32,

    #[nserde(rename = "arrayMode")]
    pub array_mode: u32,

    #[nserde(rename = "defaultTileset")]
    pub default_tileset: String,
}

#[derive(Clone, Debug, DeJson)]
pub struct Entity {
    pub name: String,
    pub limit: i32,
    pub size: Vec2i32,
    pub origin: Vec2i32,
    pub shape: Shape,
    pub color: String,
    pub rotatable: bool,
    pub tags: Vec<String>,
    pub values: Vec<Value>,
    
    #[nserde(rename = "exportID")]
    pub export_id: String,

    #[nserde(rename = "originAnchored")]
    pub origin_anchored: bool,

    #[nserde(rename = "tileX")]
    pub tile_x: bool,

    #[nserde(rename = "tileY")]
    pub tile_y: bool,

    #[nserde(rename = "tileSize")]
    pub tile_size: Vec2i32,

    #[nserde(rename = "resizeableX")]
    pub resizeable_x: bool,

    #[nserde(rename = "resizeableY")]
    pub resizeable_y: bool,

    #[nserde(rename = "rotationDegrees")]
    pub rotation_degrees: i32,

    #[nserde(rename = "canFlipX")]
    pub can_flip_x: bool,

    #[nserde(rename = "canFlipY")]
    pub can_flip_y: bool,

    #[nserde(rename = "canSetColor")]
    pub can_set_color: bool,

    #[nserde(rename = "hasNodes")]
    pub has_nodes: bool,

    #[nserde(rename = "nodeLimit")]
    pub node_limit: u32,

    #[nserde(rename = "nodeDisplay")]
    pub node_display: u32,
    
    #[nserde(rename = "nodeGhost")]
    pub node_ghost: bool,
}


#[derive(Clone, Debug, DeJson)]
pub struct Shape {
    pub label: String,
    pub points: Vec<Vec2i32>,
}

#[derive(Clone, Debug, DeJson)]
pub struct Value {
    pub name: String,
    pub definition: String,
    pub defaults: String,

    #[nserde(rename = "defaults")]
    pub default_u64: Option<u64>,

    pub bounded: Option<bool>,
    pub min: Option<u32>,
    pub max: Option<u32>,

    #[nserde(rename = "trimWhitespace")]
    pub trim_whitespace: Option<bool>,

    #[nserde(rename = "maxLength")]
    pub max_length: Option<u32>,
}

#[derive(Clone, Debug, DeJson)]
pub struct Tileset {
    pub label: String,
    pub path: String,

    #[nserde(rename = "tileSeparationX")]
    pub tile_separation_x: u32,

    #[nserde(rename = "tileSeparationY")]
    pub tile_separation_y: u32,

    #[nserde(rename = "tileWidth")]
    pub tile_width: u32,

    #[nserde(rename = "tileHeight")]
    pub tile_height: u32,

    // pub image: String,
}