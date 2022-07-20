use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, RwLock};

use realfft::RealFftPlanner;
use realfft::RealToComplex;
use rustfft::num_complex::Complex;

use crate::configuration::Configuration;
use crate::plot_data::PlotRow;
use crate::recorder::RecorderData;

pub struct WaterfallProcessor {
    receiver: Receiver<RecorderData>,
    sender: Sender<PlotRow>,
    fft: Arc<dyn RealToComplex<f32>>,
    config: Arc<RwLock<Configuration>>,
    fft_depth: usize,
}

impl WaterfallProcessor {
    pub fn new(
        receiver: Receiver<RecorderData>,
        sender: Sender<PlotRow>,
        config: Arc<RwLock<Configuration>>,
    ) -> Self {
        let mut planner = RealFftPlanner::<f32>::new();
        let fft_depth = config.read().unwrap().fft_depth;
        let fft = planner.plan_fft_forward(fft_depth);

        Self {
            receiver,
            sender,
            fft,
            fft_depth,
            config,
        }
    }

    pub fn start(&mut self) {
        let fft_depth = self.config.read().unwrap().fft_depth;
        let mut data: Vec<RecorderData> = Vec::with_capacity(fft_depth);
        loop {
            let Configuration {
                audio_sample_rate,
                fft_depth,
                min_db,
                max_db,
                trim_hz,
            } = *self.config.read().unwrap();

            if self.fft_depth != fft_depth {
                let mut planner = RealFftPlanner::<f32>::new();
                self.fft_depth = fft_depth;
                self.fft = planner.plan_fft_forward(self.fft_depth);
            }

            let sample = self.receiver.recv().unwrap();
            data.push(sample);

            if data.len() < fft_depth {
                continue;
            }

            let mut spectrum = self.fft.make_output_vec();
            self.fft
                .process(data.as_mut_slice(), &mut spectrum)
                .unwrap();

            // Bins are now Fs / N wide
            // Drop bins that are out of SSB passband
            let new_length =
                (trim_hz as f32 / (audio_sample_rate as f32 / fft_depth as f32)) as usize;
            if new_length < fft_depth {
                spectrum.resize(new_length, Complex::default());
            }

            // 30 dB is 255
            // -20 dB is 0
            // (-20, 0) -> (30, 255)
            // y = 5.1x + 102
            let m = 255.0 / (max_db - min_db);
            let scale_func = |x| m * (x - min_db);

            let normalized: Vec<u8> = spectrum
                .iter()
                .map(|c| c.norm()) // Magnitude
                .map(|f| f / (fft_depth as f32).sqrt()) // Normalization
                .map(|f| 10.0 * f.log10()) // dB
                // .map(|f| 5.1 * f + 102.0)
                .map(scale_func)
                .map(|f| f.clamp(0.0, 255.0))
                .map(|f| f as u8)
                .collect();

            self.sender.send(normalized).unwrap();

            data.clear();
        }
    }
}
