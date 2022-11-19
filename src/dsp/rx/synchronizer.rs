use crate::configuration::AudioSampleRate;
use crate::dsp::correlator::{Correlator, OperandData};
use crate::synth::ft8::SyncSignal;
use crate::types::SampleType;

pub struct Synchronizer {
    correlator: Correlator,
    sync_data: OperandData,
    sample_rate: AudioSampleRate,
}

impl Synchronizer {
    pub fn new(buffer_len: usize, sample_rate: AudioSampleRate) -> Self {
        let correlator = Correlator::with_pow2_len(buffer_len * 7);

        let SyncSignal(mut sync_signal) = SyncSignal::new(sample_rate.baseband_sample_rate());
        sync_signal.resize(correlator.input_size(), 0.0);
        let sync_data = correlator.prepare_rhs(&sync_signal);

        Self {
            correlator,
            sync_data,
            sample_rate,
        }
    }

    pub fn synchronize(&self, signal: Vec<SampleType>) -> Option<usize> {
        let input_size = self.correlator.input_size();
        let (value, position) =
            (0..signal.len() / 2)
                .into_iter()
                .step_by(64)
                .fold((0., 0), |max, start| {
                    let end = if start + input_size > signal.len() {
                        signal.len()
                    } else {
                        start + input_size
                    };

                    let lhs = self.correlator.prepare_lhs(&signal[start..end]);
                    let (value, position) =
                        self.correlator
                            .correlate_max_with_prepared(&lhs, &self.sync_data, true);

                    if value > max.0 {
                        (value, end)
                    } else {
                        max
                    }
                });

        println!("Max value of {} at {}", value, position);
        if value > 0.5 {
            Some(position * self.sample_rate.samples_to_skip())
        } else {
            None
        }
    }
}
