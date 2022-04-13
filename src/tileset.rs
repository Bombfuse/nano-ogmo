use nanoserde::DeJson;

#[derive(Clone, Debug, DeJson)]
pub struct Tileset {
    pub label: String,
    pub path: String,

    #[nserde(rename = "tileWidth")]
    pub tile_width: usize,

    #[nserde(rename = "tileHeight")]
    pub tile_height: usize,

    #[nserde(rename = "tileSeparationX")]
    pub tile_separation_x: usize,

    #[nserde(rename = "tileSeparationY")]
    pub tile_separation_y: usize,
}
