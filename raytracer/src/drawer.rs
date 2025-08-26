use crate::types::camera::Camera;
use crate::types::common::Drawable;
use crate::types::lightsource::LightSource;
extern crate sdl3;

use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::render::WindowCanvas;
use sdl3::{video::Window, EventPump, Sdl};

use std::thread;
use std::time::Duration;
use std::vec::Vec;
use std::collections::HashMap;

pub struct SdlApp {
    pub sdl_context: Sdl,
    pub video_subsystem: sdl3::VideoSubsystem,
    pub event_pump: EventPump,
    pub canvas: HashMap<String, WindowCanvas>,
    pub windows: HashMap<String, Window>,
    pub cameras: HashMap<String, Camera>,
}

pub trait Drawer {
    fn new() -> Self;
    fn add_window(&mut self, camera: &Camera);
    fn prepare_scenes(&mut self, cameras: Vec<Camera>);
    fn draw(
	&mut self,
	scene: &Vec<Box<dyn Drawable>>,
	lightsources: &Vec<LightSource>,
    );
    fn main_loop(
	&mut self,
	scene: &Vec<Box<dyn Drawable>>,
	lightsources: &Vec<LightSource>,
    );
}

impl Drawer for SdlApp {

    fn new() -> Self {
	let sdl_context = sdl3::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();
	let event_pump = sdl_context.event_pump().unwrap();
	SdlApp {
	    sdl_context: sdl_context,
	    video_subsystem: video_subsystem,
	    event_pump: event_pump,
	    canvas: HashMap::new(), 
	    windows: HashMap::new(),
	    cameras: HashMap::new(),
	}
    }
    
    fn add_window(&mut self, camera: &Camera) {

	println!("Creating window {} shape {}x{}", camera.common.name, camera.hfov, camera.vfov);
	self.windows.insert(camera.common.name.clone(),
			    self.video_subsystem
			    .window(&camera.common.name, camera.hfov as u32, camera.vfov as u32)
			    .position_centered()
			    .build()
			    .map_err(|e| e.to_string()).unwrap());
	
	// Create a canvas to draw on.
	let canvas = self.windows[&camera.common.name].clone().into_canvas();
	self.canvas.insert(camera.common.name.clone(), canvas);
	self.cameras.insert(camera.common.name.clone(), camera.clone());
    }

    fn prepare_scenes(&mut self, cameras: Vec<Camera>) {
	for camera in cameras {
	    self.add_window(&camera);
	}
    }

    fn draw(
	&mut self,
	scene: &Vec<Box<dyn Drawable>>,
	lightsources: &Vec<LightSource>,
    ) {
	for (name, window) in &self.windows {
            let canvas = self.canvas.get_mut(name).unwrap();
	    canvas.clear();
            let mut x = 0;
            while x < window.size().0 {
		let mut y = 0;
		while y < window.size().1 {
                    for item in scene {
			canvas.set_draw_color(
			    item.draw(
				&self.cameras[&name.clone()],
				x as f32,
				y as f32,
				&lightsources,
			    )
			);
			canvas.draw_point((x, y)).unwrap();
                    }
                    y = y + 1;
		}
		x = x + 1;
            }
            canvas.present();
	}
    }

    fn main_loop(
	&mut self,
	scene: &Vec<Box<dyn Drawable>>,
	lightsources: &Vec<LightSource>,
    ) {
	'running: loop {
	    for event in self.event_pump.poll_iter() {
		match event {
		    Event::Quit { .. }
		    | Event::KeyDown {
			keycode: Some(Keycode::Escape),
			..
		    } => break 'running,
		    Event::KeyDown {
			keycode: Some(Keycode::Up),
			..
		    } => {
			for (_name, camera) in &mut self.cameras {
			    camera.common.forward.0[0] += 1.0;
			}
			continue;
		    }
		    Event::KeyDown {
			keycode: Some(Keycode::Down),
			..
		    } => {
			for (_name, camera) in &mut self.cameras {
			    camera.common.forward.0[0] -= 1.0;
			}
			continue;
		    }
		    Event::KeyDown {
			keycode: Some(Keycode::Right),
			..
		    } => {
			for (_name, camera) in &mut self.cameras {
			    camera.common.right.0[0] += 1.0;
			}
			continue;
		    }
		    Event::KeyDown {
			keycode: Some(Keycode::Left),
			..
		    } => {
			for (_name, camera) in &mut self.cameras {
			    camera.common.right.0[0] -= 1.0;
			}
			continue;
		    }
		    
		    _ => {}
		}
	    }
	    self.draw(scene, lightsources);
	    thread::sleep(Duration::from_millis(10));
	}
    }
}
