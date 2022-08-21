use std::f32::consts::PI;

use crate::synth::samples::Samples;
use crate::units::Frequency;

pub struct Symbol {
    sample_rate: Frequency,
    carrier: Frequency,
    sample: u64,
    symbol: f32,
    amplitude: f32,
}

impl Symbol {
    // pub fn new(sample_rate: Frequency, carrier: Frequency, symbol: f32) -> Self {
    //     Symbol::with_amplitude(sample_rate, carrier, symbol, 0.005)
    // }

    pub fn with_amplitude(
        sample_rate: Frequency,
        carrier: Frequency,
        symbol: f32,
        amplitude: f32,
    ) -> Self {
        Self {
            sample_rate,
            carrier,
            sample: 0,
            symbol,
            amplitude,
        }
    }
}

impl Samples for Symbol {
    fn next(&mut self) -> f32 {
        let t = (self.sample as f32) / self.sample_rate.value();
        let modulation_index = 1.0;
        let signaling_interval = 0.160;
        let phase = modulation_index * self.symbol / signaling_interval;

        let val = self.amplitude * (2.0 * PI * ((self.carrier.value() + phase) * t)).sin();

        self.sample += 1;

        val
    }
}
