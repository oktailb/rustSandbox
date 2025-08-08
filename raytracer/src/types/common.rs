use glam::{vec3, Vec3};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::any::Any;

#[derive(Debug, Clone, Copy)]
pub struct Vec3Serde(pub Vec3);

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
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Common {
    pub classification: String,
    pub name: String,
    pub position: Vec3Serde,
    pub orientation: Vec3Serde,
    pub color: Color,
}

pub trait HasCommon {
    fn common(&self) -> &Common;
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
}
