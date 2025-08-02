use crate::types::common::AsPoint;
use crate::types::common::Common;
use crate::types::common::Drawable;
use crate::types::common::Point;
use sdl3::pixels::Color;
use sdl3::render::Canvas;
use sdl3::video::Window;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Square {
    pub common: Common,
    cote: f32,
}

impl AsPoint for Square {
    fn as_point(&self) -> &Point {
        &self.common.position
    }
    fn as_point_mut(&mut self) -> &mut Point {
        &mut self.common.position
    }
}

impl Drawable for Square {
    fn draw(&self, canvas: &mut Canvas<Window>) {
        println!("Je suis un carré de côté {}.", self.cote);
    }
}
