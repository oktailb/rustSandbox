use sdl3::render::Canvas;
use sdl3::video::Window;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Point {
    pub x: i32,
    pub y: i32,
    pub z: i32,
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
    pub position: Point,
    pub orientation: Point,
    pub color: Color,
}

impl Point {
    fn get_coords(&self) -> (i32, i32, i32) {
        (self.x, self.y, self.z)
    }

    fn move_delta(&mut self, dx: i32, dy: i32, dz: i32) {
        self.x += dx;
        self.y += dy;
        self.z += dz;
    }
}

pub trait AsPoint {
    fn as_point(&self) -> &Point;
    fn as_point_mut(&mut self) -> &mut Point;
}

pub trait Drawable: AsPoint {
    fn draw(&self, canvas: &mut Canvas<Window>);
    fn position(&self) -> (i32, i32, i32) {
        self.as_point().get_coords()
    }

    fn move_delta(&mut self, dx: i32, dy: i32, dz: i32) {
        self.as_point_mut().move_delta(dx, dy, dz);
    }
}

