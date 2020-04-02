use crate::synth::envelope::EnvelopeADSR;
use crate::synth::osc;
use rand::random;

pub const BASE_A: f32 = 444.0;

pub struct Note {
    pub value: u8,
    pub velocity: u8,
    pub start: f32,
    pub end: Option<f32>
}

// Generic instrument
pub trait Instrument: Send {

    fn name(&self) -> &'static str;

    fn get_sample(&self, note: &Note, time: f32) -> Option<f32>;
}

// Empty instrument
impl Instrument for () {

    fn name(&self) -> &'static str { "Empty" }

    fn get_sample(&self, note: &Note, time: f32) -> Option<f32> {
        None
    }
}

// Most simple instruments will have an ADSR envelope on amplitude and some kind of underlying
// oscillator around a base frequency

// Implementing this trait can reduce some boilerpalte
pub trait StandardInstrument: Send {
    const NAME: &'static str;

    const ENVELOPE: EnvelopeADSR;

    fn get_sample(&self, freq: f32, time: f32) -> f32;
}

impl<T: StandardInstrument> Instrument for T {

    fn name(&self) -> &'static str {
        T::NAME
    }

    fn get_sample(&self, note: &Note, time: f32) -> Option<f32> {
        let amp = match T::ENVELOPE.amplitude_at(time, note.start, note.end) {
            Some(v) => v,
            None => return None
        };

        let freq = 2f32.powf((note.value as f32 - 49.0) / 12.0) * BASE_A;

        let osc_sample = <Self as StandardInstrument>::get_sample(self, freq, time);

        Some(amp * osc_sample * (note.velocity as f32 / 127.0))
    }
}

// Standard instrument definitions

pub struct Piano;
impl StandardInstrument for Piano {
    const NAME: &'static str = "Piano";

    const ENVELOPE: EnvelopeADSR = EnvelopeADSR {
        attack_time: 0.01,
        attack_amplitude: 1.0,
        decay_time: 0.01,
        sustain_amplitude: 0.8,
        release_time: 0.02
    };

    fn get_sample(&self, freq: f32, time: f32) -> f32 {
        // 0.7 * osc::sawtooth(osc::lfo(freq, time, 1*osc::sin(5.00, time)), time)
        // + 0.1 * osc::triangle(2.0 * freq, time)
        // + 0.1 * osc::sin(4.0 * freq, time)
        // + 0.1 * osc::sawtooth(0.5 * freq, time)

        // osc::sawtooth(freq, time)
        //     + osc::sawtooth(freq * 0.5, time)
        //     + osc::sawtooth(freq * 0.249, time)

        0.7 * osc::sawtooth(osc::lfo(freq, time, 1.5* osc::sin( 2.0, time)), time)
        + 0.1 * osc::sin(freq * 2.0, time)
        + 0.15 * osc::triangle(freq * 0.249, time)

    }
}


pub struct Brass;
impl StandardInstrument for Brass {
    const NAME: &'static str = "Brass";
    const ENVELOPE: EnvelopeADSR = EnvelopeADSR {
        attack_time: 0.2,
        attack_amplitude: 1.0,
        decay_time: 0.3,
        sustain_amplitude: 0.7,
        release_time: 0.02
    };

    fn get_sample(&self, freq: f32, time: f32) -> f32 {
        osc::sawtooth(freq, time)
        + osc::sawtooth(freq * 0.5, time)
        + osc::sawtooth(freq * 0.249, time)
    }
}

pub struct DrumKit;
impl StandardInstrument for DrumKit {
    const NAME: &'static str = "Drum Kit";

    const ENVELOPE: EnvelopeADSR = EnvelopeADSR {
        attack_time: 0.01,
        attack_amplitude: 1.0,
        decay_time: 0.15,
        sustain_amplitude: 0.0,
        release_time: 0.0
    };

    fn get_sample(&self, freq: f32, time: f32) -> f32 {
        0.6 * rand::random::<f32>() * 2.0 - 1.0
    }
}

