mod level;
mod project;
mod tileset;

pub use level::*;
pub use project::*;

use nanoserde::DeJson;

#[derive(Clone, Debug, DeJson, PartialEq)]
pub struct Vec2f32 {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Debug, DeJson, PartialEq)]
pub struct Vec2i32 {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone)]
pub struct OgmoError {
    message: String,
}
impl OgmoError {
    pub fn new<T: Into<String>>(msg: T) -> Self {
        OgmoError {
            message: msg.into(),
        }
    }
}

impl std::convert::From<nanoserde::DeJsonErr> for OgmoError {
    fn from(e: nanoserde::DeJsonErr) -> OgmoError {
        OgmoError {
            message: e.to_string(),
        }
    }
}
