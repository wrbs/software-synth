pub struct EnvelopeADSR {
    pub attack_time: f32,
    pub attack_amplitude: f32,
    pub decay_time: f32,
    pub sustain_amplitude: f32,
    pub release_time: f32,
}

impl EnvelopeADSR {
    pub fn amplitude_at(&self, time: f32, on_time: f32, off_time: Option<f32>) -> Option<f32> {

        let off_time = off_time.unwrap_or(std::f32::INFINITY);

        if time < on_time {
            return Some(0.0);
        }

        let amplitude = if time < off_time {
            let lifetime = time - on_time;
            if lifetime <= self.attack_time {
                lifetime / self.attack_time * self.attack_amplitude
            } else if lifetime <= self.attack_time + self.decay_time {
                self.attack_amplitude -  (lifetime - self.attack_time) / self.decay_time * (self.attack_amplitude - self.sustain_amplitude)
            } else {
                self.sustain_amplitude
            }
        } else {
            let deadtime = time - off_time;
            let amplitude_at_trigger_off = match self.amplitude_at(off_time, on_time, None) {
                Some(x) => x,
                None => return None
            };

            if deadtime < self.release_time {
                amplitude_at_trigger_off * (1.0 - deadtime / self.release_time)
            } else {
                return None;
            }
        };

        Some(if amplitude < 0.001 {
            0.0
        } else {
            amplitude
        })
    }
}
