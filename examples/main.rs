extern crate sdl2;

use core::default::Default;
use sdl2::event::Event;
use sdl2::sys::SDL_tan;
use ugui::control::{Button, Control};
use ugui::standard_styler::StandardStyler;
use ugui::*;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Test", 640, 480)
        .opengl()
        .resizable()
        .build()
        .map_err(|e| e.to_string())?;

    let canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump().map_err(|e| e.to_string())?;

    let mut ugui = Ugui {
        current_input: Default::default(),
        last_input: Default::default(),
        styler: Box::new(StandardStyler { canvas }),
    };

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => {}
            }
        }

        ugui.begin(Input {
            x: 0f32,
            y: 0f32,
            primary_down: false,
        });

        ugui.button(
            Control {
                uid: 0,
                enabled: true,
                rect: geo::Rect::new(60, 30, 100, 23),
            },
            Button {
                text: "Hi".parse().unwrap(),
            },
        );

        ugui.end();

        //
        // let rect = Rect::new(
        //     event_pump.mouse_state().x(),
        //     event_pump.mouse_state().y(),
        //     100,
        //     100,
        // );
        // canvas.set_draw_color(Color::RGB(255, 255, 255));
        // canvas.draw_rect(rect).unwrap();
        // canvas.fill_rect(rect).unwrap();
        // canvas.present();
    }

    Ok(())
}
