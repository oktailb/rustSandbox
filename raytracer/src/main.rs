mod conf;
mod draw;
mod types;

use conf::build_drawables_from_json;
use draw::draw;
use std::env;
extern crate sdl3;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let rt_file = &args[1];

        let (scene, utils) = build_drawables_from_json(rt_file)?;
        draw(Ok(scene), Ok(utils));
    } else {
        println!("Usage: {} ...", &args[0]);
    }
    Ok(())
}
