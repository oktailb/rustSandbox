use crate::types::common::HasCommon;
use crate::types::common::Common;
use crate::types::common::Drawable;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::any::Any;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Circle {
    pub common: Common,
    pub radius: f32,
}

impl HasCommon for Circle {
    fn common(&self) -> &Common {
        &self.common
    }
}

impl Drawable for Circle {
    fn draw(&self, cam: &crate::types::camera::Camera, x: f32, y: f32, lightsources: &Result<Vec<Box<dyn Drawable>>, String>) -> sdl3::pixels::Color {
	let mut ret = sdl3::pixels::Color::RGBA(0, 0, 0, 0);
        let center = self.common.position;
	
        let radius = self.radius;

        let dist_x = (center.0.x - x).abs();
        let dist_y = (center.0.y - y).abs();
        let dist = (dist_x * dist_x + dist_y * dist_y).sqrt();
        if dist <= radius {
	    ret = sdl3::pixels::Color::RGBA(self.common.color.r,
					    self.common.color.g,
					    self.common.color.b,
					    self.common.color.a
	    );
        }
	ret
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub fn factory(v: Value) -> Result<Box<dyn Drawable>, String> {
    let circle: Circle = serde_json::from_value(v).map_err(|e| e.to_string())?;
    Ok(Box::new(circle))
}
