use std::cmp::Ordering;

use crate::dsp::correlator::{Correlator, OperandData};
use crate::units::Frequency;
use crate::synth::ft8::SingleSymbol;
use crate::types::SampleType;
use crate::synth::Samples;

pub struct Symbolizer {
    symbols: Vec<OperandData>,
    correlator: Correlator,
}

impl Symbolizer {
    pub fn new(buffer_len: usize, baseband_sample_rate: Frequency) -> Self {
        let correlator = Correlator::with_pow2_len(buffer_len);

        let carrier = Frequency::Hertz(0.0);
        let mut symbols: Vec<OperandData> = Vec::with_capacity(8);

        for symbol in 0..8 {
            let mut gen =
                SingleSymbol::with_amplitude(baseband_sample_rate, carrier, symbol as f32, 1.0);

            // let len: usize =
            //     (sample_rate.value() / (carrier.value() + (symbol as f32) * 6.25)) as usize;
            let syn: Vec<SampleType> = (0..correlator.output_size())
                .into_iter()
                .map(|_| gen.next())
                .collect();

            symbols.push(correlator.prepare_rhs(&syn));
        }

        Self {
            symbols,
            correlator,
        }
    }

    pub fn symbolize(&self, signal: Vec<SampleType>) -> () {
        let lhs = self.correlator.prepare_lhs(&signal);
        let _symbol = self
            .symbols
            .iter()
            .map(|syn| {
                self.correlator
                    .correlate_max_with_prepared(&lhs, syn, true)
                    .0
            })
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .map(|(index, _)| index)
            .unwrap();
    }
}
