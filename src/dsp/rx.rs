use std::cmp::Ordering;

use tokio::sync::mpsc::error::TrySendError;
use tokio::sync::mpsc::Sender;
use wasm_timer::Instant;

use crate::configuration::Configuration;
use crate::dsp::aggregator::Aggregator;
use crate::dsp::correlator::{Correlator, OperandData};
use crate::filter::{BandPassFilter, Filter, LowPassFilter};
use crate::statistics::{MovingAverage, Statistics};
use crate::synth::Symbol;
use crate::synth::{Samples, Sine};
use crate::types::SampleType;
use crate::units::Frequency;

pub struct Rx {
    symbols: Vec<OperandData>,
    correlator: Correlator,
    aggregator: Aggregator,
    sample_rate: Frequency,
    plot_sender: Sender<Vec<SampleType>>,
    downsample_skip: usize,
}

impl Rx {
    pub fn new(plot_sender: Sender<Vec<SampleType>>, config: &Configuration) -> Self {
        let mut symbols: Vec<OperandData> = Vec::with_capacity(8);

        let sample_rate_raw = config.audio_sample_rate;
        let sample_rate = Frequency::Hertz(sample_rate_raw as f32);
        let baseband_sample_rate = match sample_rate_raw {
            8000 => Frequency::Hertz(8000.0),
            16000 => Frequency::Hertz(8000.0),
            22050 => Frequency::Hertz(11025.0),
            44100 => Frequency::Hertz(11025.0),
            48000 => Frequency::Hertz(12000.0),
            96000 => Frequency::Hertz(12000.0),
            _ => sample_rate,
        };

        let downsample_skip: usize = sample_rate_raw / (baseband_sample_rate.value() as usize);

        let carrier = Frequency::Hertz(0.0);

        let aggregator_len = (sample_rate.value() / 6.25) as usize;
        let aggregator = Aggregator::new(aggregator_len);

        let buffer_len = (baseband_sample_rate.value() / 6.25) as usize;
        let ideal_buffer_len = 2_f32.powf((buffer_len as f32).log2().ceil()) as usize;

        let correlator = Correlator::new(ideal_buffer_len);

        for symbol in 0..8 {
            let mut gen = Symbol::with_amplitude(baseband_sample_rate, carrier, symbol as f32, 1.0);

            // let len: usize =
            //     (sample_rate.value() / (carrier.value() + (symbol as f32) * 6.25)) as usize;
            let syn: Vec<SampleType> = (0..ideal_buffer_len)
                .into_iter()
                .map(|_| gen.next())
                .collect();

            symbols.push(correlator.prepare_rhs(&syn));
        }
        Self {
            symbols,
            correlator,
            aggregator,
            sample_rate,
            plot_sender,
            downsample_skip,
        }
    }

    pub fn run(
        &mut self,
        new_samples: Vec<SampleType>,
        config: &Configuration,
        stats: &mut Statistics,
    ) {
        let sample_rate = Frequency::Hertz(config.audio_sample_rate as f32);
        if sample_rate.value() != self.sample_rate.value() {
            *self = Self::new(self.plot_sender.clone(), config);
        }

        self.aggregator.aggregate(new_samples);

        while let Some(samples) = self.aggregator.get_slice() {
            let now = Instant::now();

            // Bandpass
            let mut bpf1 = BandPassFilter::from_frequency(
                config.tuner.lower_absolute(), // Low
                config.tuner.upper_absolute(), // High
                self.sample_rate,              // SampleRate
            );
            let mut bpf2 = BandPassFilter::from_frequency(
                config.tuner.lower_absolute(), // Low
                config.tuner.upper_absolute(), // High
                self.sample_rate,              // SampleRate
            );
            let mut bpf3 = BandPassFilter::from_frequency(
                config.tuner.lower_absolute(), // Low
                config.tuner.upper_absolute(), // High
                self.sample_rate,              // SampleRate
            );
            let bp1 = samples.into_iter().map(|sample| bpf1.next(sample));
            let bp2 = bp1.map(|sample| bpf2.next(sample));
            let bandpassed = bp2.map(|sample| bpf3.next(sample));

            // LO Mix
            // let if_carrier = config.tuner.carrier() - Frequency::Hertz(100.001);
            let if_carrier = config.tuner.carrier();
            let mut carrier = Sine::new(self.sample_rate, if_carrier);
            let mixed = bandpassed.map(|sample| sample * carrier.next());

            // Low Pass
            let mut lpf = LowPassFilter::from_frequency(
                Frequency::Hertz(1000.0),
                self.sample_rate, // SampleRate
            );
            let low_passed = mixed.map(|sample| lpf.next(sample));

            // Collect into signal
            let signal: Vec<SampleType> = low_passed.step_by(self.downsample_skip).collect();

            if let Err(err) = self.plot_sender.try_send(signal.clone()) {
                match err {
                    TrySendError::Full(_) => println!("Plot ui falling behind"),
                    TrySendError::Closed(_) => (),
                }
            }

            // Correlate
            let lhs = self.correlator.prepare_lhs(&signal);
            let symbol = self
                .symbols
                .iter()
                .map(|syn| self.correlator.correlate_max_with_prepared(&lhs, syn, true))
                .enumerate()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
                .map(|(index, _)| index)
                .unwrap();

            let elapsed = now.elapsed();
            stats.rx.push(elapsed);
            // println!("Decoded: {}", symbol);
        }
    }
}
