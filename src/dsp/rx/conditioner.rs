use crate::configuration::Configuration;
use crate::filter::{Filter, BandPassFilter, LowPassFilter};
use crate::synth::{Samples, Sine};
use crate::types::SampleType;
use crate::units::Frequency;

pub struct Conditioner {
    downsample_skip: usize,
}

impl Conditioner {
    pub fn new() -> Self {
        Self {
            downsample_skip: 1,
        }
    }

    pub fn with_downsample_skip(self, downsample_skip: usize) -> Self {
        Self {
            downsample_skip
        }
    }

    pub fn condition(&self, config: &Configuration, samples: &Vec<SampleType>) -> Vec<SampleType> {
        let sample_rate = Frequency::Hertz(config.audio_sample_rate as f32);

        // Bandpass
        let mut bpf = BandPassFilter::from_frequency(
            config.tuner.lower_absolute(), // Low
            config.tuner.upper_absolute(), // High
            sample_rate,              // SampleRate
        );
        let bandpassed = samples.iter().map(|sample| bpf.next(*sample));

        // LO Mix
        let if_carrier = config.tuner.carrier();
        let mut carrier = Sine::new(sample_rate, if_carrier);
        let mixed = bandpassed.map(|sample| sample * carrier.next());

        // Low Pass
        let mut lpf = LowPassFilter::from_frequency(
            Frequency::Hertz(1000.0),
            sample_rate, // SampleRate
        );
        let low_passed = mixed.map(|sample| lpf.next(sample));

        // Decimate
        let decimated = low_passed.step_by(self.downsample_skip);

        // Collect into signal
        let signal: Vec<SampleType> = decimated.collect();

        return signal;
    }
}
