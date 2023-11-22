extern crate sdl2;

use core::default::Default;
use sdl2::event::Event;
use ugui::control::{Button, Control, Listbox, Scrollbar};
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
        styler: StandardStyler::new(canvas, &ttf_context),
        persistent_state: Default::default(),
    };

    let mut mouse_wheel = 0;
    let mut value = 0.0;
    let mut index = Some(0);
    let items = vec![
        "Item A", "Item B", "Item C", "Item A", "Item B", "Item C", "Item A", "Item B", "Item C",
        "Item A", "Item B", "Item C", "Item A", "Item B", "Item C", "Item A", "Item B", "Item C",
        "Item A", "Item B", "Item C",
    ];

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::MouseWheel { y, .. } => {
                    mouse_wheel = y as i8;
                }
                _ => {}
            }
        }

        ugui.begin(Input {
            mouse_position: Point {
                x: event_pump.mouse_state().x() as f32,
                y: event_pump.mouse_state().y() as f32,
            },
            mouse_wheel,
            primary_down: event_pump.mouse_state().left(),
        });
        mouse_wheel = 0;

        if ugui.button(
            Control {
                uid: 0,
                enabled: true,
                rect: geo::Rect::new(60.0, 30.0, 100.0, 23.0),
            },
            Button {
                text: &*index.unwrap().to_string(),
            },
        ) {
            println!("the")
        }

        index = ugui.listbox(
            Control {
                uid: 1,
                enabled: true,
                rect: geo::Rect::new(60.0, 80.0, 200.0, 350.0),
            },
            Listbox {
                items: &items,
                index,
            },
        );

        value = ugui.scrollbar(
            Control {
                uid: 2,
                enabled: true,
                rect: geo::Rect::new(300.0, 80.0, 16.0, 350.0),
            },
            Scrollbar { value, ratio: 1.5 },
        );

        ugui.end();
    }

    Ok(())
}
