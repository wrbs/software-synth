use std::sync::mpsc::Receiver;
use crate::Message;
use crate::output::Source;
use crate::synth::instrument::{Instrument, Note};
use std::collections::HashMap;


mod instrument;
mod envelope;
mod osc;


pub struct Synth {
    chan: Receiver<Message>,
    // Hardcoded channel limit
    channels: [Box<dyn Instrument>; 16],
    channel_volumes: [f32; 16],
    notes: HashMap<(u8, u8), Note>,
    todo: Option<Vec<(f32, Message)>>,
}

impl Synth {
    pub fn new(chan: Receiver<Message>, mut todo: Option<Vec<(f32, Message)>>) -> Synth {
        todo = todo.map(|mut x| { x.reverse(); x });
        Synth {
            chan,
            channels: [
                // 1-9
                Box::new(instrument::Piano),
                Box::new(instrument::Piano),
                Box::new(instrument::Piano),
                Box::new(instrument::Piano),
                Box::new(instrument::Piano),
                Box::new(instrument::Piano),
                Box::new(instrument::Piano),
                Box::new(instrument::Piano),
                Box::new(instrument::Piano),
                // 10
                Box::new(instrument::DrumKit),
                // 11-
                Box::new(instrument::Piano),
                Box::new(instrument::Piano),
                Box::new(instrument::Piano),
                Box::new(instrument::Piano),
                Box::new(instrument::Piano),
                Box::new(instrument::Piano),
            ],
            notes: HashMap::new(),
            channel_volumes: [1.0; 16],
            todo
        }
    }
}

impl Source for Synth {
    fn update(&mut self, time: f32) {
        let mut to_handle = Vec::new();
        if let Some(todo_stack) = self.todo.as_mut() {
            loop {
                if let Some((t, m)) = todo_stack.last() {
                    if *t <= time {
                        let (t, m) = todo_stack.pop().unwrap();
                        to_handle.push((t, m));
                        continue;
                    }
                }

                break;
            }
        }

        for m in self.chan.try_iter() {
            to_handle.push((time, m))
        }

        for (time, msg) in to_handle {
            match msg {
                Message::KeyDown { channel: c, note: n, velocity } => {
                    println!("{:.2}: {:?}", time, msg);
                    assert!(c < 16);
                    self.notes.insert((c, n), Note {
                        value: n,
                        start: time,
                        end: None,
                        velocity
                    });
                },
                Message::KeyUp { channel: c, note: n } => {
                    assert!(c < 16);
                    // Set the end time if the note is currently being played
                    self.notes.entry((c, n))
                        .and_modify(|e| e.end = Some(time));
                },
                Message::ProgramChange { channel: c, program} => {
                    println!("{:.2}: {:?}", time, msg);

                    // Percussion, always (thanks GM)
                    if c != 9 /* channel 10 */ {

                        let instr: Box<dyn Instrument> = match program {
                            // Piano
                            16 => Box::new(instrument::B),
                            91 => Box::new(instrument::C),
                            60 => Box::new(instrument::Brass),
                            0..=7 => Box::new(instrument::Piano),
                            8..=127 => Box::new(instrument::Piano),
                            _ => panic!("unreachable"),
                        };

                        // println!("{:?} TO {}", msg, instr.name());

                        self.channels[c as usize] = instr;
                    }
                },
                Message::VolumeChange { channel: c, volume} => {
                    println!("{:.2}: {:?}", time, msg);
                    self.channel_volumes[c as usize] = (volume as f32 / 127.0)
                }
            }
        }
    }

    fn get_sample(&mut self, t: f32) -> f32 {
        let mut to_remove = Vec::new();
        let mut v = 0.0;
        for ((c, k), n) in self.notes.iter() {
            let cnum = (*c) as usize;
            match self.channels[cnum].get_sample(n, t) {
                Some(x) => v += 0.1 * x * self.channel_volumes[cnum],
                None => to_remove.push((*c, *k)),
            }
        }

        for k in to_remove {
            self.notes.remove(&k);
        }

        v
    }
}

