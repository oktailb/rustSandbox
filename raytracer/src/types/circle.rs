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
pub struct Circle {
    pub common: Common,
    pub radius: f32,
}

impl AsPoint for Circle {
    fn as_point(&self) -> &Point {
        &self.common.position
    }
    fn as_point_mut(&mut self) -> &mut Point {
        &mut self.common.position
    }
}

impl Drawable for Circle {
    fn draw(&self, canvas: &mut Canvas<Window>) {
        // We'll use the position from the AsPoint trait
        let center_x = self.as_point().x as f32;
        let center_y = self.as_point().y as f32;
        let radius = self.radius;

        canvas.set_draw_color(Color::RGB(
            self.common.color.r,
            self.common.color.g,
            self.common.color.b,
        ));

        let mut x = center_x - radius;

        while x < (center_x + radius) {
            let mut y = center_y - radius;
            while y < (center_y + radius) {
                // Draw the 8 symmetric points of the circle
                let dist_x = (center_x - x).abs();
                let dist_y = (center_y - y).abs();
                let dist = (dist_x * dist_x + dist_y * dist_y).sqrt();
                if dist <= radius {
                    canvas.draw_point((x, y)).unwrap();
                }
                y = y + 1.0;
            }
            x = x + 1.0;
        }
    }
}

pub fn factory(v: Value) -> Result<Box<dyn Drawable>, String> {
    let circle: Circle = serde_json::from_value(v).map_err(|e| e.to_string())?;
    Ok(Box::new(circle))
}
