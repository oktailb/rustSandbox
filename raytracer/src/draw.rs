use crate::types::common::Drawable;
extern crate sdl3;

use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::{EventPump, Sdl, render::Canvas, video::Window};

pub struct SdlApp {
    pub sdl_context: Sdl,
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump,
}

pub fn init(width: u32, height: u32) -> Result<SdlApp, String> {
    let sdl_context = sdl3::init().map_err(|e| e.to_string())?;
    let video_subsystem = sdl_context.video().map_err(|e| e.to_string())?;

    let window = video_subsystem
        .window("SDL3 Rust Example", width, height)
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

pub fn draw(scene: Result<Vec<Box<dyn Drawable>>, String>, app: &mut SdlApp) {
    app.canvas.set_draw_color(sdl3::pixels::Color::RGB(0, 0, 0)); // Black background
    app.canvas.clear();
    match scene {
        Ok(list) => {
            for item in list {
                item.draw(&mut app.canvas);
            }
        }
        Err(e) => {
            eprintln!("Error on reading: {}", e);
        }
    }
    app.canvas.present();
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

