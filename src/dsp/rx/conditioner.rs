use crate::configuration::Configuration;
use crate::filter::{BandPassFilter, Filter, LowPassFilter};
use crate::synth::{Samples, Sine};
use crate::types::SampleType;
use crate::units::Frequency;

pub struct Conditioner {}

impl Conditioner {
    pub fn new() -> Self {
        Self {}
    }

    pub fn condition(&self, config: &Configuration, samples: &Vec<SampleType>) -> Vec<SampleType> {
        let sample_rate = config.audio_sample_rate;

        let copied = samples.iter().copied();

        // Bandpass
        // let mut bpf = BandPassFilter::from_frequency(
        //     config.tuner.lower_absolute(), // Low
        //     config.tuner.upper_absolute(), // High
        //     sample_rate.into(),            // SampleRate
        // );
        // let bandpassed = samples.iter().map(|sample| bpf.next(*sample));

        // // LO Mix
        // let if_carrier = config.tuner.carrier();
        // let mut carrier = Sine::new(sample_rate.into(), if_carrier);
        // let mixed = bandpassed.map(|sample| sample * carrier.next());

        // // Low Pass
        // let mut lpf = LowPassFilter::from_frequency(
        //     Frequency::Hertz(1000.0),
        //     sample_rate.into(), // SampleRate
        // );
        // let low_passed = mixed.map(|sample| lpf.next(sample));

        // Decimate
        let decimated = copied.step_by(sample_rate.samples_to_skip());

        // Collect into signal
        let signal: Vec<SampleType> = decimated.collect();

        return signal;
    }
}
