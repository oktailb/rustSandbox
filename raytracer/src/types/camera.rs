use crate::types::common::Common;
use crate::types::common::Drawable;
use crate::types::common::HasCommon;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::any::Any;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
#[derive(Clone)]
pub struct Camera {
    pub common: Common,
    pub hfov: f32,
    pub vfov: f32,
    pub dist: f32,
    pub fps: u32,
}

impl Camera {
    pub fn ray_for_pixel(&self, px: f32, py: f32) -> crate::types::common::Ray {
        // px et py sont normalisés entre [-1, 1] avec (0,0) au centre de l'écran
        let x_offset = px * (self.hfov / 2.0).tan() * self.dist;
        let y_offset = py * (self.vfov / 2.0).tan() * self.dist;

        // Point sur le plan image
        let target = self.common.position.to_vec3()
            + self.common.forward.to_vec3() * self.dist
            + self.common.right.to_vec3() * x_offset
            + self.common.up.to_vec3() * y_offset;

        // Direction normalisée
        let dir = (target - self.common.position.to_vec3()).normalize();

        crate::types::common::Ray {
            origin: self.common.position.to_vec3(),
            direction: dir,
        }
    }
}

impl HasCommon for Camera {
    fn common(&self) -> &Common {
        &self.common
    }
}

impl Drawable for Camera {
    fn draw(
        &self,
        _cam: &crate::types::camera::Camera,
        _x: f32,
        _y: f32,
        _lightsources: &Result<Vec<Box<dyn Drawable>>, String>,
    ) -> sdl3::pixels::Color {
        sdl3::pixels::Color::RGBA(self.common.color.r,
				  self.common.color.g,
				  self.common.color.b,
				  self.common.color.a)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub fn factory(v: Value) -> Result<Box<dyn Drawable>, String> {
    let camera: Camera = serde_json::from_value(v).map_err(|e| e.to_string())?;
    Ok(Box::new(camera))
}
