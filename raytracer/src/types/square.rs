use crate::types::common::AsPoint;
use crate::types::common::Common;
use crate::types::common::Drawable;
use crate::types::common::Point;
use sdl3::pixels::Color;
use sdl3::render::Canvas;
use sdl3::video::Window;
use serde::{Deserialize, Serialize};
use serde_json::Value;

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
        let coord_x = self.as_point().x;
        let coord_y = self.as_point().y;
        let cote = self.cote as i32;

        canvas.set_draw_color(Color::RGB(
            self.common.color.r,
            self.common.color.g,
            self.common.color.b,
        ));
	let mut x = coord_x;
	
        while x < (coord_x + cote) {
	    let mut y = coord_y;
            while y < (coord_y + cote) {
                canvas.draw_point((x, y)).unwrap();
                y = y + 1;
            }
            x = x + 1;
        }
    }
}

pub fn factory(v: Value) -> Result<Box<dyn Drawable>, String> {
    let square: Square = serde_json::from_value(v).map_err(|e| e.to_string())?;
    Ok(Box::new(square))
}
