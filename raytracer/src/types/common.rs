use glam::Vec3;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::any::Any;

#[derive(Debug, Clone, Copy)]
pub struct Vec3Serde(pub Vec3);

impl Vec3Serde {
    pub fn to_vec3(&self) -> Vec3 {
        Vec3::new(self.0.x, self.0.y, self.0.z)
    }

    /*
    pub fn from_vec3(v: Vec3) -> Self {
        Vec3Serde { x: v.x, y: v.y, z: v.z }
}
     */
}

impl Serialize for Vec3Serde {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let arr = [self.0.x, self.0.y, self.0.z];
        arr.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Vec3Serde {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let arr: [f32; 3] = Deserialize::deserialize(deserializer)?;
        Ok(Vec3Serde(Vec3::new(arr[0], arr[1], arr[2])))
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
#[derive(Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
#[derive(Clone)]
pub struct Common {
    pub classification: String,
    pub name: String,
    pub position: Vec3Serde,
    pub up: Vec3Serde,
    pub forward: Vec3Serde,
    pub right: Vec3Serde,
    pub color: Color,
}

pub trait HasCommon {
    fn common(&self) -> &Common;
}

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<f32>; // distance à l'intersection
}

pub trait Drawable: HasCommon {
    fn draw(
        &self,
        cam: &crate::types::camera::Camera,
        x: f32,
        y: f32,
        lightsources: &Result<Vec<Box<dyn Drawable>>, String>,
    ) -> sdl3::pixels::Color;

    fn classification(&self) -> &str {
        &self.common().classification
    }

    fn as_any(&self) -> &dyn Any;

    fn shade(
        &self,
        hit_point: Vec3,
        normal: Vec3,
        lightsources: &Result<Vec<Box<dyn Drawable>>, String>
    ) -> sdl3::pixels::Color {
        let base_color = sdl3::pixels::Color::RGBA(255, 255, 255, 255);

        if let Ok(lights) = lightsources {
            let mut intensity = 0.0;

            for light in lights {
                let pos = light.common().position.to_vec3();

                let light_dir = (pos - hit_point).normalize();
                let diff = normal.dot(light_dir).max(0.0);
                intensity += diff;
                
            }

            intensity = intensity.clamp(0.0, 1.0);
            sdl3::pixels::Color::RGBA(
                (base_color.r as f32 * intensity) as u8,
                (base_color.g as f32 * intensity) as u8,
                (base_color.b as f32 * intensity) as u8,
                (base_color.a as f32 * intensity) as u8
            )
        } else {
            base_color
        }
    }
}

