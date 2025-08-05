mod conf;
mod draw;
mod types;

use conf::build_drawables_from_json;
use draw::{draw, event_loop, init};
use std::env;
extern crate sdl3;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let rt_file = &args[1];

        let mut app = init(800, 600)?;
        let scene = build_drawables_from_json(rt_file);
        draw(scene, &mut app);
        event_loop(&mut app);
    } else {
        println!("Usage: {} ...", &args[0]);
    }
    Ok(())
}
