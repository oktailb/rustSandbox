use crate::types::common::Drawable;
extern crate sdl3;

use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::{render::Canvas, video::Window, EventPump, Sdl};

pub struct SdlApp {
    pub sdl_context: Sdl,
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump,
}

pub fn init(width: u32, height: u32, title: &String) -> Result<SdlApp, String> {
    let sdl_context = sdl3::init().map_err(|e| e.to_string())?;
    let video_subsystem = sdl_context.video().map_err(|e| e.to_string())?;

    let window = video_subsystem
        .window(&title, width, height)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    // Create a canvas to draw on.
    let canvas = window.into_canvas();

    // Get the event pump for handling user input.
    let event_pump = sdl_context.event_pump().map_err(|e| e.to_string())?;

    // Return the new struct containing all the components.
    Ok(SdlApp {
        sdl_context,
        canvas,
        event_pump,
    })
}

pub fn draw(
    scene: Result<Vec<Box<dyn Drawable>>, String>,
    cameras: Result<Vec<Box<dyn Drawable>>, String>,
    lightsources: Result<Vec<Box<dyn Drawable>>, String>,
) {
    match cameras {
        Ok(environement) => {
            for item in environement {
                if item.classification() == "Camera" {
                    if let Some(camera) =
                        item.as_any().downcast_ref::<crate::types::camera::Camera>()
                    {
                        let mut app = init(camera.hfov as u32, camera.vfov as u32, &camera.common.name).unwrap();

                        app.canvas.set_draw_color(sdl3::pixels::Color::RGBA(camera.common.color.r,
									    camera.common.color.g,
									    camera.common.color.b,
									    camera.common.color.a));
                        app.canvas.clear();
			let mut x = 0.0;
			while x < camera.hfov {
			    let mut y = 0.0;
			    while y < camera.vfov {
				match scene {
				    Ok(ref list) => {
					for item in list {
					    app.canvas.set_draw_color(item.draw(camera, x, y, &lightsources));
					    app.canvas.draw_point((x, y)).unwrap();
					}
				    }
				    Err(ref e) => {
					eprintln!("Error on reading objects: {}", e);
				    }
				}
				y = y + 1.0;
			    }
			    x = x + 1.0;
			}
                        app.canvas.present();
                        event_loop(&mut app);
                    } else {
                        println!("-> Downcast failed.");
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error on reading utils: {}", e);
        }
    }
}

pub fn event_loop(app: &mut SdlApp) {
    let event_pump = &mut app.event_pump;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
    }
}
