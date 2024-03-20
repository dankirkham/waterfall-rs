use std::sync::Arc;

use realfft::RealFftPlanner;
use realfft::RealToComplex;

use crate::units::{Frequency, Time};

pub struct Fft {
    fft: Arc<dyn RealToComplex<f32>>,
    pub depth: usize,
    pub sample_rate: Frequency,
}

impl Fft {
    pub fn new(duration: Time, sample_rate: Frequency) -> Self {
        assert_ne!(duration.value(), 0.0);
        assert_ne!(sample_rate.value(), 0.0);
        let mut planner = RealFftPlanner::<f32>::new();

        dbg!(duration / sample_rate);
        let depth = (duration / sample_rate).round() as usize;
        let fft = planner.plan_fft_forward(depth as usize);

        Self {
            depth,
            fft,
            sample_rate,
        }
    }

    /// aka delta_f or df
    fn bin_hz(&self) -> Frequency {
        self.sample_rate / self.depth as f32
    }

    pub fn bin_to_frequency(&self, bin: usize) -> Frequency {
        self.bin_hz() * bin
    }

    pub fn frequency_to_bin(&self, f: Frequency) -> usize {
        (f / self.bin_hz()) as usize
    }

    pub fn run(&self, mut samples: Vec<f32>) -> Vec<f32> {
        assert_eq!(samples.len(), self.depth);
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
