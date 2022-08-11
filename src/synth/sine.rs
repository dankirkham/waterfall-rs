use std::f32::consts::PI;

use crate::units::Frequency;

pub struct Sine {
    sample_rate: Frequency,
    frequency: Frequency,
    amplitude: f32,
    sample: u64,
}

impl Sine {
    pub fn new(sample_rate: Frequency, frequency: Frequency, amplitude: f32) -> Self {
        Self {
            sample_rate,
            frequency,
            amplitude,
            sample: 0,
        }
    }

    pub fn next(&mut self) -> f32 {
        let t = (self.sample as f32) / self.sample_rate.value();

        let val = self.amplitude * (2.0 * PI * self.frequency.value() * t).sin();

        self.sample += 1;

        val
    }
}
