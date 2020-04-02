mod synth;
mod output;
mod ui;
mod midi_file_reader;

use std::sync::mpsc::{channel, Sender};
use ghakuf::messages::*;
use ghakuf::reader::*;
use std::path;

use std::thread;
use std::time::Duration;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "software-synth", about = "Procrastionation.")]
struct Opt {

    /// Input midi file (optional)
    #[structopt(parse(from_os_str))]
    input: Option<PathBuf>,

    /// Midi channel to only play
    #[structopt(short = "s", long = "solo")]
    solo:  Option<u8>

}


#[derive(Debug)]
pub enum Message {
    KeyDown { channel: u8, note: u8, velocity: u8 },
    KeyUp { channel: u8, note: u8 },
    ProgramChange { channel: u8, program: u8 },
    VolumeChange { channel: u8, volume: u8 },

}

fn main() {
    let opt = Opt::from_args();
    let (tx, rx) = channel();

    let solo = opt.solo;
    let todo = opt.input.map(|p| {
        midi_file_reader::get_messages(&p, solo)
    });

    let s = synth::Synth::new(rx, todo);
    let _s = output::start_playing(s );

    ui::start(tx);

    
}
