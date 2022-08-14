use rand::Rng;

use crate::synth::ft8::Symbol;
use crate::synth::Samples;
use crate::units::Frequency;

pub struct Ft8 {
    sample_rate: Frequency,
    carrier: Frequency,
    sample: u64,
    symbol: u8,
    amplitude: f32,
    synth: Symbol,
}

impl Ft8 {
    pub fn new(sample_rate: Frequency, carrier: Frequency) -> Self {
        let signaling_interval = 0.160;
        let symbol = 0;
        let synth = Symbol::new(sample_rate, carrier, symbol as f32);

        Self {
            sample_rate,
            carrier,
            sample: (sample_rate.value() * signaling_interval) as u64,
            symbol,
            amplitude: 0.001,
            synth,
        }
    }
}

impl Samples for Ft8 {
    fn next(&mut self) -> f32 {
        if self.sample == 0 {
            let signaling_interval = 0.160;
            self.sample = (self.sample_rate.value() * signaling_interval) as u64;

            // self.symbol += 1;
            // if self.symbol > 7 {
            //     self.symbol = 0;
            // }

            let mut rng = rand::thread_rng();
            self.symbol = rng.gen_range(0..8);

            self.synth = Symbol::new(self.sample_rate, self.carrier, self.symbol as f32);
        }

        self.sample -= 1;

        self.synth.next()
    }
}
