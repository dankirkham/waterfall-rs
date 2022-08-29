use std::f32::consts::PI;

use crate::synth::samples::Samples;
use crate::units::Frequency;

pub struct Sine {
    sample_rate: Frequency,
    frequency: Frequency,
    amplitude: f32,
    sample: u64,
}

impl Sine {
    pub fn new(sample_rate: Frequency, frequency: Frequency) -> Self {
        Self::with_amplitude(sample_rate, frequency, 0.001)
    }

    pub fn with_amplitude(sample_rate: Frequency, frequency: Frequency, amplitude: f32) -> Self {
        Self {
            sample_rate,
            frequency,
            amplitude,
            sample: 0,
        }
    }
}

impl Samples for Sine {
    fn next(&mut self) -> f32 {
        let t = (self.sample as f32) / self.sample_rate.value();

        let val = self.amplitude * (2.0 * PI * self.frequency.value() * t).cos();

        self.sample += 1;

        val
    }
}
