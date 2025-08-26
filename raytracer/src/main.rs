mod conf;
mod drawer;
mod types;

use conf::build_drawables_from_json;
use drawer::{Drawer, SdlApp};
use std::env;
extern crate sdl3;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let rt_file = &args[1];

        let (scene, cameras, lightsources) = build_drawables_from_json(rt_file)?;
	let mut drawer = SdlApp::new();
	drawer.prepare_scenes(cameras);
	drawer.main_loop(&scene, &lightsources);

    } else {
        println!("Usage: {} ...", &args[0]);
    }
    Ok(())
}
