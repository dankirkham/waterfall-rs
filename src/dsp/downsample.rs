use std::f32::consts::PI;

use crate::dsp::fir::{AsymmetricFir, FirBuilder};
use crate::units::Frequency;

#[derive(Debug)]
pub struct Downsample {
    lpf: AsymmetricFir,
    factor: u32,
    counter: u32,
    pub output_sample_rate: Frequency,
}

impl Downsample {
    pub fn new(
        input_sample_rate: Frequency,
        output_sample_rate: Frequency,
        kernel_length: usize,
    ) -> Self {
        assert!(input_sample_rate.value() > output_sample_rate.value());
        assert_eq!(kernel_length % 2, 1);

        let factor = input_sample_rate.value() / output_sample_rate.value();
        let factor = factor as u32;

        let lpf =
            FirBuilder::low_pass(101, input_sample_rate, output_sample_rate).build_asymmetric();

        let output_sample_rate = Frequency::Hertz(input_sample_rate.value() / factor as f32);

        Self {
            lpf,
            factor,
            counter: 0,
            output_sample_rate,
        }
    }

    pub fn update(&mut self, input: f32) -> Option<f32> {
        let next = self.lpf.update(input);

        if self.counter == 0 {
            self.counter = self.factor - 1;
            Some(next)
        } else {
            self.counter -= 1;
            None
        }
    }
}
