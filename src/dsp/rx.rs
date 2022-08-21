use std::mem;
use std::sync::mpsc::Sender;

use crate::configuration::Configuration;
use crate::filter::{BandPassFilter, Filter, LowPassFilter};
use crate::recorder::RecorderData;
use crate::synth::Symbol;
use crate::synth::{Samples, Sine};
use crate::units::Frequency;

use super::correlation::correlate;

pub struct Rx {
    symbols: Vec<Vec<RecorderData>>,
    plot: Sender<Vec<RecorderData>>,
    buffer_len: usize,
    data: Vec<RecorderData>,
}

impl Rx {
    pub fn new(plot: Sender<Vec<RecorderData>>) -> Self {
        let mut symbols: Vec<Vec<RecorderData>> = Vec::with_capacity(8);

        let sample_rate = Frequency::Hertz(44100.0);
        let carrier = Frequency::Hertz(100.0);

        let buffer_len = (sample_rate.value() / 6.25) as usize;
        let data: Vec<RecorderData> = Vec::with_capacity(buffer_len);

        for symbol in 0..8 {
            let mut gen = Symbol::with_amplitude(sample_rate, carrier, symbol as f32, 1.0);

            let len: usize =
                (sample_rate.value() / (carrier.value() + (symbol as f32) * 6.25)) as usize;
            let syn: Vec<RecorderData> = (0..len).into_iter().map(|_| gen.next()).collect();
            symbols.push(syn);
        }
        Self {
            symbols,
            plot,
            buffer_len,
            data,
        }
    }

    pub fn run(&mut self, new_samples: Vec<RecorderData>, config: Configuration) {
        if self.data.len() < self.buffer_len {
            self.data.extend(new_samples);
            return;
        }

        while self.data.len() > self.buffer_len {
            let mut subset: Vec<RecorderData>;
            let samples = {
                subset = Vec::with_capacity(self.buffer_len);
                subset.extend_from_slice(&self.data[0..self.buffer_len]);
                self.data.rotate_left(self.buffer_len);
                self.data.resize(self.data.len() - self.buffer_len, 0.0);
                subset
            };

            let sample_rate = Frequency::Hertz(config.audio_sample_rate as f32);

            // Bandpass
            let mut bpf = BandPassFilter::from_frequency(
                config.tuner.lower_absolute(), // Low
                config.tuner.upper_absolute(), // High
                sample_rate,                   // SampleRate
            );
            let bandpassed = samples.iter().map(|sample| bpf.next(*sample));

            // LCO Mix
            let if_carrier = config.tuner.carrier - Frequency::Hertz(100.001);
            let mut carrier = Sine::new(sample_rate, if_carrier);
            let mixed = bandpassed.map(|sample| sample * carrier.next());

            // Low Pass
            let mut lpf = LowPassFilter::from_frequency(
                Frequency::Hertz(1000.0),
                sample_rate, // SampleRate
            );
            let low_passed = mixed.map(|sample| lpf.next(sample));

            // Collect into signal
            let signal: Vec<RecorderData> = low_passed.collect();

            // Correlate
            let mut max_symbol = 0;
            let mut total_max = 0.0;
            for symbol in 0..8 {
                let syn = &self.symbols[symbol];

                let c = correlate(&signal, &syn, true);

                if symbol == 0 {
                    self.plot.send(signal.clone());
                }

                let max = c
                    .into_iter()
                    //.map(|v| v.abs())
                    //.sum();
                    .fold(-f32::INFINITY, |a, b| a.max(b));
                // println!("{}: {}", symbol, max);

                if max > total_max {
                    total_max = max;
                    max_symbol = symbol;
                }
                // println!("{}: {}", symbol, max);
            }

            // println!("Decoded: {} ({})", max_symbol, total_max);
        }
    }
}
