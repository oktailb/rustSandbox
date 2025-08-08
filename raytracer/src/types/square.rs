use crate::types::common::HasCommon;
use crate::types::common::Common;
use crate::types::common::Drawable;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::any::Any;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Square {
    pub common: Common,
    cote: f32,
}

impl HasCommon for Square {
    fn common(&self) -> &Common {
        &self.common
    }
}

impl Drawable for Square {
    fn draw(&self, cam: &crate::types::camera::Camera, x: f32, y: f32, lightsources: &Result<Vec<Box<dyn Drawable>>, String>) -> sdl3::pixels::Color {
        let coord = self.common.position;
        let cote = self.cote as i32;

        sdl3::pixels::Color::RGBA(
            self.common.color.r,
            self.common.color.g,
            self.common.color.b,
            self.common.color.a,
        )
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub fn factory(v: Value) -> Result<Box<dyn Drawable>, String> {
    let square: Square = serde_json::from_value(v).map_err(|e| e.to_string())?;
    Ok(Box::new(square))
}
