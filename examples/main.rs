extern crate sdl2;

use core::default::Default;
use sdl2::event::Event;
use ugui::control::{Button, Control};
use ugui::geo::Point;
use ugui::input::Input;
use ugui::standard_styler::StandardStyler;
use ugui::*;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    let window = video_subsystem
        .window("Test", 640, 480)
        .opengl()
        .resizable()
        .build()
        .map_err(|e| e.to_string())?;

    let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump().map_err(|e| e.to_string())?;

    let mut ugui = Ugui {
        active_control: None,
        styler: StandardStyler::new(canvas, &ttf_context),
        current_input: Default::default(),
        last_input: Default::default(),
        mouse_down_position: Default::default(),
    };

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => {}
            }
        }

        ugui.begin(Input {
            mouse_position: Point {
                x: event_pump.mouse_state().x() as f32,
                y: event_pump.mouse_state().y() as f32,
            },
            primary_down: event_pump.mouse_state().left(),
        });

        ugui.button(
            Control {
                uid: 0,
                enabled: true,
                rect: geo::Rect::new(60.0, 30.0, 100.0, 23.0),
            },
            Button {
                text: &"Hi".to_string(),
            },
        );

        ugui.end();
    }

    Ok(())
}
