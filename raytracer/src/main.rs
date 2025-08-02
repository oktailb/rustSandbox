use serde::Deserialize;
use std::error::Error;
use std::path::Path;
mod conf;
mod draw;
mod types;

use conf::build_drawables_from_json;
use draw::{draw, event_loop, init};
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
extern crate sdl3;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

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
