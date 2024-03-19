use std::sync::Arc;

use rustfft::{Fft, FftPlanner, num_complex::Complex};

fn make_output_vec(samples: &[f32]) -> Vec<Complex<f32>> {
    samples.iter().map(|&n| Complex::<f32> { re: n, im: 0.0 }).collect()
}

pub struct Ifft {
    ifft: Arc<dyn Fft<f32>>,
    pub depth: usize,
}

impl Ifft {
    pub fn new(depth: usize) -> Self {
        let mut planner = FftPlanner::new();
        let ifft = planner.plan_fft_inverse(depth);

        Self {
            ifft,
            depth,
        }
    }

    pub fn run(&self, samples: &[f32]) -> Vec<Complex<f32>> {
        assert_eq!(self.depth, samples.len());
        let mut spectrum = make_output_vec(&samples);
        self.ifft.process(&mut spectrum);

        let spectrum: Vec<_> = spectrum
            .into_iter()
            .map(|f| f / (self.depth as f32).sqrt()) // Normalization
            .collect();

        spectrum
    }
}
