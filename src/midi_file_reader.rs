use std::sync::mpsc::{channel, Sender};
use ghakuf::messages::*;
use ghakuf::reader::*;
use std::path;
use super::Message;
use std::path::Path;

use std::thread;
use std::time::Duration;


pub fn get_messages(path: &Path, solo: Option<u8>) -> Vec<(f32, Message)> {
    let mut handler = MidiHandler { micros_per_quarter_note: 500000, ticks_per_quarter_note: 0, events: Vec::new(), t: 0.0 };
    let mut reader = Reader::new(&mut handler, &path).unwrap();
    let _ = reader.read();

    let mut events = handler.events;
    events.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());

    let mut messages = Vec::new();

    for (t, e) in events {
        match e {
            MidiEvent::NoteOff { ch, note, .. } => {
                if solo == None || solo == Some(ch) {
                    messages.push((t, Message::KeyUp { channel: ch, note }));
                }
            },
            MidiEvent::NoteOn { ch, note, velocity } => {
                if solo == None || solo == Some(ch) {
                    if velocity == 0 {
                        messages.push((t, Message::KeyUp { channel: ch, note }));
                    } else {
                        messages.push((t, Message::KeyDown { channel: ch, note, velocity }));
                    }
                }
            },
            MidiEvent::ProgramChange { ch, program } => {
                if solo == None || solo == Some(ch) {
                    messages.push((t, Message::ProgramChange { channel: ch, program }));
                }
            },
            MidiEvent::ControlChange { ch, control, data } => {
                if solo == None || solo == Some(ch) {
                    match control {
                        7 => messages.push((t, Message::VolumeChange { channel: ch, volume: data })),
                        _ => ()
                    }
                }
            }
            _ => {}
        }
    }

    messages
}

struct MidiHandler {
    micros_per_quarter_note: u32,
    ticks_per_quarter_note: u16,
    events: Vec<(f32, MidiEvent)>,
    t: f32,
}

impl Handler for MidiHandler {
    fn header(&mut self, format: u16, track: u16, time_base: u16) {
        self.ticks_per_quarter_note = time_base;
        println!("TICKS: {} FORMAT: {}", self.ticks_per_quarter_note, format);
        let _ = (format, track);
    }
    
    fn meta_event(&mut self, delta_time: u32, event: &MetaEvent, data: &Vec<u8>) {
        let _ = (delta_time, event, data);
        if event == &MetaEvent::SetTempo {
            let mut val = 0;
            for b in data.iter() {
                val <<= 8;
                val |= (*b) as u32;
            }
            self.micros_per_quarter_note = val;
            println!("{:?} TEMPO: {} BPM: {}", data, self.micros_per_quarter_note, 60.0e6 / self.micros_per_quarter_note as f32);
        }
    }
    
    fn midi_event(&mut self, delta_time: u32, event: &MidiEvent) {
        let delta_secs = delta_time as f32 * (self.micros_per_quarter_note as f32 / self.ticks_per_quarter_note as f32);

        self.t += delta_secs / 1.0e6;

        self.events.push((self.t, event.clone()));

    }
//    fn sys_ex_event(&mut self, delta_time: u32, event: &SysExEvent, data: &Vec<u8>) {
//        let _ = (delta_time, event, data);
//    }
    fn track_change(&mut self) {
        self.t = 0.0;
        // self.micros_per_quarter_note = 500000;
    }
}
