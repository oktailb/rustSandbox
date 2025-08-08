use crate::types::common::Common;
use crate::types::common::Drawable;
use crate::types::common::HasCommon;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::any::Any;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Camera {
    pub common: Common,
    pub hfov: u32,
    pub vfov: u32,
    pub fps: u32,
}

impl HasCommon for Camera {
    fn common(&self) -> &Common {
        &self.common
    }
}

impl Drawable for Camera {
    fn draw(
        &self,
        cam: &crate::types::camera::Camera,
        x: f32,
        y: f32,
        lightsources: &Result<Vec<Box<dyn Drawable>>, String>,
    ) -> sdl3::pixels::Color {
        sdl3::pixels::Color::RGBA(0, 0, 0, 0)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub fn factory(v: Value) -> Result<Box<dyn Drawable>, String> {
    let camera: Camera = serde_json::from_value(v).map_err(|e| e.to_string())?;
    Ok(Box::new(camera))
}
