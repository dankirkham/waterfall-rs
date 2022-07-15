use std::sync::mpsc::{Sender, Receiver};
use std::sync::Arc;
use std::cmp::Ordering;

use itertools::Itertools;

use realfft::RealFftPlanner;
use realfft::RealToComplex;
use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;

use crate::recorder::RecorderData;
use crate::plot_data::{PlotRow, PLOT_WIDTH};

const N: usize = 6000;
// const N: usize = 44100;
// const N: usize = 1024;

pub struct WaterfallProcessor {
    receiver: Receiver<RecorderData>,
    sender: Sender<PlotRow>,
    fft: Arc<dyn RealToComplex<f32>>,
}

impl WaterfallProcessor {
    pub fn new(receiver: Receiver<RecorderData>, sender: Sender<PlotRow>) -> Self {
        let mut planner = RealFftPlanner::<f32>::new();
        let fft = planner.plan_fft_forward(N);
        Self { receiver, sender, fft }
    }

    pub fn start(&self) {
        let mut data: Vec<RecorderData> = Vec::with_capacity(N);
        loop {
            let sample = self.receiver.recv().unwrap();
            data.push(sample);

            if data.len() < N {
                continue;
            }

            let mut spectrum = self.fft.make_output_vec();
            self.fft.process(data.as_mut_slice(), &mut spectrum);

            // Bins are now Fs / N wide
            // Drop bins that are out of SSB passband (3kHz)
            let new_length: usize = 3000 / (44100 / N);
            spectrum.resize(new_length, Complex::default());

            // 30 dB is 255
            // -20 dB is 0
            // (-20, 0) -> (30, 255)
            // y = 5.1x + 102

            let normalized: Vec<u8> = spectrum
                .iter()
                .map(|c| c.norm()) // Magnitude
                .map(|f| f / (N as f32).sqrt()) // Normalization
                .map(|f| 10.0 * f.log10()) // dB
                .map(|f| 5.1 * f + 102.0)
                .map(|f| f.clamp(0.0, 255.0))
                .map(|f| f as u8)
                .collect();

            self.sender.send(normalized).unwrap();

            data.clear();
        }
    }
}
