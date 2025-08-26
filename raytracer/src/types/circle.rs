use crate::types::common::Common;
use crate::types::common::Drawable;
use crate::types::common::HasCommon;
use crate::types::common::Intersectable;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::any::Any;

use super::lightsource::LightSource;

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
    fn draw(
        &self,
        cam: &crate::types::camera::Camera,
        x: f32,
        y: f32,
        lightsources: &Vec<LightSource>,
    ) -> sdl3::pixels::Color {
        let ray = cam.ray_for_pixel(x, y);

        if let Some(t) = self.intersect(&ray) {
            let hit_point = ray.origin + ray.direction * t;
            let normal = self.common.forward.to_vec3().normalize();
            let color = self.shade(hit_point, normal, lightsources);
            //	    println!("{}x{} ...", x ,y);

            color
        } else {
            sdl3::pixels::Color::RGBA(0, 0, 0, 255)
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Intersectable for Circle {
    fn intersect(&self, ray: &crate::types::common::Ray) -> Option<f32> {
        let center = self.common.position.to_vec3();
        let normal = self.common.forward.to_vec3().normalize();
        let ray_origin = ray.origin;
        let ray_dir = ray.direction.normalize();
        let denom = normal.dot(ray_dir);
        if denom.abs() < 1e-6 {
            return None; // rayon parallèle au disque
        }

        let t = (center - ray_origin).dot(normal) / denom;
        if t < 0.0 {
            return None; // intersection derrière l'origine
        }

        // Point d'intersection
        let hit_point = ray_origin + ray_dir * t;
        let dist_to_center = (hit_point - center).length();
        if dist_to_center <= self.radius {
            Some(t)
        } else {
            None
        }
    }
}

pub fn factory(v: Value) -> Result<Box<dyn Drawable>, String> {
    let circle: Circle = serde_json::from_value(v).map_err(|e| e.to_string())?;
    Ok(Box::new(circle))
}
