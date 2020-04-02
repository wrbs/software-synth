use std::sync::mpsc::Sender;
use crate::Message;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
use std::collections::{HashSet, HashMap};
use std::thread;
use std::time::Duration;


pub fn start(tx: Sender<Message>) {
    println!(" _______________________________________________\n\
              || | |  |  | | | |  |  | | | | | |  |  | | | |  |\n\
              || | |  |  | | | |  |  | | | | | |  |  | | | |  |\n\
              || | |  |  | | | |  |  | | | | | |  |  | | | |  |\n\
              || |W|  |  |R| |T|  |  |U| |I| |O|  |  |[| |]|  |\n\
              |   |   |   |   |   |   |   |   |   |   |   |   |\n\
              | A | S | D | F | G | H | J | K | L | ; | ' | # |\n\
              |___|___|___|___|___|___|___|___|___|___|___|___|");

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Will's synth", 100, 100)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(255, 255, 255));

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut current_channel = 0;
    let mut current_keys = HashSet::new();

    'running: loop {
        canvas.clear();
        canvas.present();

        let key_state = event_pump.keyboard_state();

        let mut next_keys = HashSet::new();
        for (scancode, id) in &[
            (Scancode::A, 37),
            (Scancode::W, 38),
            (Scancode::S, 39),
            (Scancode::D, 40),
            (Scancode::R, 41),
            (Scancode::F, 42),
            (Scancode::T, 43),
            (Scancode::G, 44),
            (Scancode::H, 45),
            (Scancode::U, 46),
            (Scancode::J, 47),
            (Scancode::I, 48),
            (Scancode::K, 49),
            (Scancode::O, 50),
            (Scancode::L, 51),
            (Scancode::Semicolon, 52),
            (Scancode::LeftBracket, 53),
            (Scancode::Apostrophe, 54),
            (Scancode::RightBracket, 55),
            (Scancode::Backslash, 56),
        ] {
            if key_state.is_scancode_pressed(*scancode) {
                next_keys.insert(*id);
            }
        }

        let mut to_remove = HashSet::new();

        for key in current_keys.iter() {
            if !next_keys.contains(key) {
                tx.send(Message::KeyUp { channel: current_channel, note: *key }).unwrap();
                to_remove.insert(*key);
            } else {
                next_keys.remove(key);
            }
        }

        // Now current keys only contains the keys not currently pressed
        for key in next_keys {
            tx.send(Message::KeyDown { channel: current_channel, note: key, velocity: 100 }).unwrap();
            current_keys.insert(key);
        }

        for k in to_remove {
            current_keys.remove(&k);
        }

        let mut next_channel = current_channel;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                Event::KeyDown { keycode: Some(Keycode::Up ), .. } => {
                    next_channel = (next_channel + 1) % 16;
                },
                Event::KeyDown { keycode: Some(Keycode::Down ), .. } => {
                    next_channel = if next_channel == 0 {
                        15
                    } else {
                        next_channel - 1
                    };
                }
                _ => ()
            }
        }

        if next_channel != current_channel {
            for key in current_keys.iter() {
                tx.send(Message::KeyUp { channel: current_channel, note: *key }).unwrap();
                tx.send(Message::KeyDown { channel: next_channel, note: *key, velocity: 100 }).unwrap();
            }

            println!("Now on channel {}", next_channel);
        }

        current_channel = next_channel;

        thread::sleep(Duration::from_millis(2));
    }
}
