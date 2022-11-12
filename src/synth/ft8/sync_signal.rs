use crate::synth::ft8::message::sync_sequence;
use crate::synth::ft8::Symbol;
use crate::synth::samples::Samples;
use crate::types::SampleType;
use crate::units::Frequency;

#[derive(Clone)]
pub struct SyncSignal(pub Vec<SampleType>);

impl SyncSignal {
    pub fn new(sample_rate: Frequency) -> Self {
        let signaling_interval = 0.160;
        let sample_count = (sample_rate.value() * signaling_interval) as usize;
        let sequence = sync_sequence();

        let mut synth = Symbol::with_amplitude(sample_rate, Frequency::Hertz(0.0), 0.0, 0.005);
        let mut signal: Vec<SampleType> = Vec::with_capacity(sample_count * 7);

        (0..7).into_iter().for_each(|index| {
            synth.set_symbol(sequence[index].into());
            (0..sample_count).into_iter().for_each(|_| {
                signal.push(synth.next());
            });
        });

        // There is an integrator, so we must rotate it so that the first
        // symbol starts right away.
        // let mut rotated_signal: Vec<SampleType> = Vec::with_capacity(signal.len());
        // let half_signal_len = sample_count / 2;
        // rotated_signal.extend_from_slice(&signal[half_signal_len..]);
        // rotated_signal.extend_from_slice(&signal[..half_signal_len]);

        Self(signal)
    }
}
