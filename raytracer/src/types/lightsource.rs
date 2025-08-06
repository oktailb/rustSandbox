use crate::types::common::HasCommon;
use crate::types::common::AsPoint;
use crate::types::common::Common;
use crate::types::common::Drawable;
use crate::types::common::Point;
use sdl3::render::Canvas;
use sdl3::video::Window;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::any::Any;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct LightSource {
    pub common: Common,
    mode: String,
}

impl AsPoint for LightSource {
    fn as_point(&self) -> &Point {
        &self.common.position
    }
    fn as_point_mut(&mut self) -> &mut Point {
        &mut self.common.position
    }
}

impl HasCommon for LightSource {
    fn common(&self) -> &Common {
        &self.common
    }
}

impl Drawable for LightSource {
    fn draw(&self, canvas: &mut Canvas<Window>) {
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub fn factory(v: Value) -> Result<Box<dyn Drawable>, String> {
    let lightsource: LightSource = serde_json::from_value(v).map_err(|e| e.to_string())?;
    Ok(Box::new(lightsource))
}
