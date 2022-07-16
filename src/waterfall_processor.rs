use std::sync::mpsc::{Sender, Receiver};
use std::sync::Arc;
use std::cmp::Ordering;

use itertools::Itertools;

use realfft::RealFftPlanner;
use realfft::RealToComplex;
use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;

use crate::configuration::{GlobalConfig, Configuration};
use crate::recorder::RecorderData;
use crate::plot_data::{PlotRow, PLOT_WIDTH};

pub struct WaterfallProcessor {
    receiver: Receiver<RecorderData>,
    sender: Sender<PlotRow>,
    fft: Arc<dyn RealToComplex<f32>>,
    config: GlobalConfig,
    fft_depth: usize,
}

impl WaterfallProcessor {
    pub fn new(receiver: Receiver<RecorderData>, sender: Sender<PlotRow>, config: GlobalConfig) -> Self {
        let mut planner = RealFftPlanner::<f32>::new();
        let fft_depth = config.read().unwrap().fft_depth;
        let fft = planner.plan_fft_forward(fft_depth);

        Self { receiver, sender, fft, config, fft_depth }
    }

    fn update_config(&mut self) -> Configuration {
        let config = self.config.read().unwrap().clone();
        let fft_depth = config.fft_depth;
        if fft_depth != self.fft_depth {
            let mut planner = RealFftPlanner::<f32>::new();
            self.fft = planner.plan_fft_forward(fft_depth);
            self.fft_depth = fft_depth;
        }
        config
    }

    pub fn start(&mut self) {
        let Configuration {
            audio_sample_rate,
            fft_depth,
            min_db,
            max_db,
            trim_hz,
        } = self.update_config();
        let mut data: Vec<RecorderData> = Vec::with_capacity(fft_depth);
        loop {
            let sample = self.receiver.recv().unwrap();
            data.push(sample);

            if data.len() < fft_depth {
                continue;
            }

            let mut spectrum = self.fft.make_output_vec();
            self.fft.process(data.as_mut_slice(), &mut spectrum);

            // Bins are now Fs / N wide
            // Drop bins that are out of SSB passband
            let new_length: usize = trim_hz / (audio_sample_rate / fft_depth);
            if new_length < fft_depth {
                spectrum.resize(new_length, Complex::default());
            }

            // 30 dB is 255
            // -20 dB is 0
            // (-20, 0) -> (30, 255)
            // y = 5.1x + 102

            let normalized: Vec<u8> = spectrum
                .iter()
                .map(|c| c.norm()) // Magnitude
                .map(|f| f / (fft_depth as f32).sqrt()) // Normalization
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
