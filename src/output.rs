use cpal::traits::{HostTrait, DeviceTrait, StreamTrait};
use cpal::SampleRate;


pub const SAMPLE_RATE: u32 = 44100;

pub trait Source {
    fn update(&mut self, time: f32);
    fn get_sample(&mut self, time: f32) -> f32;
}

pub fn start_playing<S: Source + Send + 'static>(mut s: S) -> impl StreamTrait
{
    let host = cpal::default_host();
    let out = host.default_output_device().expect("Cannot open output device");
    let configs = out.supported_output_configs().expect("Could not get output formats");

    // try to find a SAMPLE_RATE stream - 44100 should exist on all systems I care about

    let sr = SampleRate(SAMPLE_RATE);
    let config = configs.filter(|x|
        x.min_sample_rate() <= sr && x.max_sample_rate() >= sr
    ).next().expect("Could not find a good stream config")
        .with_sample_rate(sr);

    let mut count = 0;
    let channels = config.channels();

    let input_data = move |data: &mut [f32]| {
        s.update(count as f32 / SAMPLE_RATE as f32 / channels as f32);
        for sample in data.iter_mut() {
            let t = count as f32 / SAMPLE_RATE as f32 / channels as f32;
            count += 1;
            let v = s.get_sample(t);
            for _ in 0..channels {
                *sample = v;
            }
        }
    };

    let stream = out.build_output_stream(
        &config.into(),
        input_data,
        |e| eprintln!("Stream error: {:?}", e)
    ).expect("Could not create stream");

    stream.play().unwrap();

    stream
}
