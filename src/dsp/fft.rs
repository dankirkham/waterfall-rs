use std::sync::Arc;

use realfft::RealFftPlanner;
use realfft::RealToComplex;

use crate::units::{Frequency, Time};

pub struct Fft {
    fft: Arc<dyn RealToComplex<f32>>,
    pub depth: usize,
    bin_hz: f32,
}

impl Fft {
    pub fn new(duration: Time, sample_rate: Frequency) -> Self {
        let mut planner = RealFftPlanner::<f32>::new();

        let depth = (duration / sample_rate) as usize;
        let fft = planner.plan_fft_forward(depth as usize);
        let bin_hz = sample_rate.value() / depth as f32;

        Self {
            depth,
            bin_hz,
            fft,
        }
    }

    pub fn bin_to_frequency(&self, bin: usize) -> Frequency {
        Frequency::Hertz(bin as f32 * self.bin_hz)
    }

    pub fn run(&self, mut samples: Vec<f32>) -> Vec<f32> {
        let mut spectrum = self.fft.make_output_vec();
        self.fft.process(&mut samples, &mut spectrum).unwrap();

        let spectrum: Vec<_> = spectrum
            .into_iter()
            .map(|c| c.norm()) // Magnitude
            .map(|f| f / (self.depth as f32).sqrt()) // Normalization
            .collect();

        spectrum
    }

}
