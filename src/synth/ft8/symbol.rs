use std::f32::consts::PI;

use statrs::function::erf::erf;

use crate::synth::samples::Samples;
use crate::units::Frequency;

pub struct Symbol {
    sample_rate: Frequency,
    carrier: Frequency,
    sample: u64,
    symbol: f32,
    previous_symbol: f32,
    amplitude: f32,
    phi: f32,
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
            previous_symbol: 0.0,
            amplitude,
            phi: 0.0,
        }
    }

    pub fn set_symbol(&mut self, symbol: f32) {
        self.previous_symbol = self.symbol;
        self.symbol = symbol;
    }
}

impl Samples for Symbol {
    fn next(&mut self) -> f32 {
        let signaling_interval = 0.160; // big T
        let t1 = (self.sample as f32) / self.sample_rate.value() - signaling_interval;
        let t = (self.sample as f32) / self.sample_rate.value();
        let modulation_index = 1.0; // little h
        let k = 5.336;

        let p1 = (1.0 / (2.0 * signaling_interval)) *
            (
                (erf((k * 2.0 * (t1 / signaling_interval + 0.5)) as f64) as f32) -
                (erf((k * 2.0 * (t1 / signaling_interval - 0.5)) as f64) as f32)
            );
        let p = (1.0 / (2.0 * signaling_interval)) *
            (
                (erf((k * 2.0 * (t / signaling_interval + 0.5)) as f64) as f32) -
                (erf((k * 2.0 * (t / signaling_interval - 0.5)) as f64) as f32)
            );
        let f_d = modulation_index * (self.symbol * p1 + self.previous_symbol * p);
        self.phi += 2.0 * PI * f_d / self.sample_rate.value();
        if self.phi > PI {
            self.phi -= 2.0 * PI;
        }

        let val = self.amplitude * (2.0 * PI * self.carrier.value() * t + self.phi).cos();

        self.sample += 1;

        if self.sample == (self.sample_rate.value() * signaling_interval) as u64 {
            self.sample = 0;
        }

        val
    }
}
