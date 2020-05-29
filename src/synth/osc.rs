use std::f32::consts::PI;

pub fn lfo(f: f32, t: f32, val: f32) -> f32 {
    f + val / 2.0 / PI / t
}

pub fn sin(f: f32, t: f32) -> f32 {
    (w(f) * t).sin()
}

pub fn square(f: f32, t: f32) -> f32 {
    if (2.0 * f * t) as usize % 2 == 0 {
        1.0
    } else {
        -1.0
    }
}

pub fn triangle(f: f32, t: f32) -> f32 {
    (w(f) * t).sin().asin() * 2.0 / PI
}

pub fn sawtooth(f: f32, t: f32) -> f32 {
    2.0 * f * (t % (1.0 / f ))  - 1.0
}

fn w(f: f32) -> f32 {
    return f * 2.0 * PI;
}
