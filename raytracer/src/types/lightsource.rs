use crate::types::common::Common;
use crate::types::common::Drawable;
use crate::types::common::HasCommon;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::any::Any;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct LightSource {
    pub common: Common,
    mode: String,
}

impl HasCommon for LightSource {
    fn common(&self) -> &Common {
        &self.common
    }
}

impl Drawable for LightSource {
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
    let lightsource: LightSource = serde_json::from_value(v).map_err(|e| e.to_string())?;
    Ok(Box::new(lightsource))
}
