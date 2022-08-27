use std::cmp::Ordering;

use rayon::prelude::*;

use crate::configuration::Configuration;
use crate::dsp::aggregator::Aggregator;
use crate::dsp::correlator::{Correlator, OperandData};
use crate::filter::{BandPassFilter, Filter, LowPassFilter};
use crate::recorder::RecorderData;
use crate::synth::Symbol;
use crate::synth::{Samples, Sine};
use crate::units::Frequency;

pub struct Rx {
    symbols: Vec<OperandData>,
    correlator: Correlator,
    aggregator: Aggregator,
}

impl Rx {
    pub fn new() -> Self {
        let mut symbols: Vec<OperandData> = Vec::with_capacity(8);

        let sample_rate = Frequency::Hertz(44100.0);
        let carrier = Frequency::Hertz(100.0);

        let buffer_len = (sample_rate.value() / 6.25) as usize;
        let correlator = Correlator::new(buffer_len);
        let aggregator = Aggregator::new(buffer_len);

        for symbol in 0..8 {
            let mut gen = Symbol::with_amplitude(sample_rate, carrier, symbol as f32, 1.0);

            let len: usize =
                (sample_rate.value() / (carrier.value() + (symbol as f32) * 6.25)) as usize;
            let syn: Vec<RecorderData> = (0..len).into_iter().map(|_| gen.next()).collect();

            symbols.push(correlator.prepare_rhs(&syn));
        }
        Self {
            symbols,
            correlator,
            aggregator,
        }
    }

    pub fn run(&mut self, new_samples: Vec<RecorderData>, config: Configuration) {
        self.aggregator.aggregate(new_samples);

        while let Some(samples) = self.aggregator.get_slice() {
            let sample_rate = Frequency::Hertz(config.audio_sample_rate as f32);

            // Bandpass
            let mut bpf = BandPassFilter::from_frequency(
                config.tuner.lower_absolute(), // Low
                config.tuner.upper_absolute(), // High
                sample_rate,                   // SampleRate
            );
            let bandpassed = samples.into_iter().map(|sample| bpf.next(sample));

            // LO Mix
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
            let lhs = self.correlator.prepare_lhs(&signal);
            let symbol = self
                .symbols
                .par_iter()
                .map(|syn| self.correlator.correlate_max_with_prepared(&lhs, syn, true))
                .enumerate()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
                .map(|(index, _)| index)
                .unwrap();

            // println!("Decoded: {}", symbol);
        }
    }
}
