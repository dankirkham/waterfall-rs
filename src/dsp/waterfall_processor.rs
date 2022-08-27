use std::collections::VecDeque;
use std::sync::{Arc, RwLock};
use std::sync::mpsc::Sender;

use egui::{Color32, ColorImage};
use realfft::RealFftPlanner;
use realfft::RealToComplex;
use rustfft::num_complex::Complex;

use crate::configuration::Configuration;
use crate::dsp::aggregator::Aggregator;
use crate::types::{PLOT_DEPTH, SampleType};

use super::turbo::get_color;

pub struct WaterfallProcessor {
    fft: Arc<dyn RealToComplex<f32>>,
    config: Arc<RwLock<Configuration>>,
    fft_depth: usize,
    image: Option<ColorImage>,
    pixels: VecDeque<Color32>,
    scroll: f32,
    aggregator: Aggregator,
    sender: Sender<ColorImage>,
    // plot: Sender<Vec<SampleType>>,
}

impl WaterfallProcessor {
    pub fn new(
        config: Arc<RwLock<Configuration>>,
        sender: Sender<ColorImage>,
    ) -> Self {
        let mut planner = RealFftPlanner::<f32>::new();
        let fft_depth = config.read().unwrap().fft_depth;
        let scroll = config.read().unwrap().scroll;
        let fft = planner.plan_fft_forward(fft_depth);
        let aggregator = Aggregator::new(fft_depth);

        Self {
            fft,
            fft_depth,
            config,
            image: None,
            pixels: VecDeque::new(),
            scroll,
            // plot,
            aggregator,
            sender,
        }
    }

    pub fn run(&mut self, new_samples: Vec<SampleType>) {
        let config = *self.config.read().unwrap();
        self.aggregator.aggregate(new_samples);

        while let Some(mut samples) = self.aggregator.get_slice() {
            if let Some(image) = &self.image {
                if image.size[0] != config.effective_len() {
                    let image =
                        ColorImage::new([config.effective_len(), PLOT_DEPTH], Color32::default());
                    self.pixels = VecDeque::from(vec![Color32::BLACK; config.effective_len() * PLOT_DEPTH]);
                    self.scroll = config.scroll;
                    self.image = Some(image);
                }
            } else {
                let image = ColorImage::new([config.effective_len(), PLOT_DEPTH], Color32::default());
                self.pixels = VecDeque::from(vec![Color32::BLACK; config.effective_len() * PLOT_DEPTH]);
                self.scroll = config.scroll;
                self.image = Some(image);
            }

            let mut spectrum = self.fft.make_output_vec();
            self.fft
                .process(&mut samples, &mut spectrum)
                .unwrap();

            if config.effective_len() < self.fft_depth {
                spectrum.resize(config.effective_len(), Complex::default());
            }

            let m = 255.0 / (config.max_db - config.min_db);
            let scale_func = |x| m * (x - config.min_db);

            self.pixels.drain(..config.effective_len());

            spectrum
                .into_iter()
                .map(|c| c.norm()) // Magnitude
                .map(|f| f / (self.fft_depth as f32).sqrt()) // Normalization
                .map(|f| 10.0 * f.log10()) // dB
                .map(scale_func)
                .map(|f| f.clamp(0.0, 255.0))
                .map(|f| f as usize)
                .map(get_color)
                .map(|[r, g, b]| Color32::from_rgb(r, g, b))
                .for_each(|pixel| self.pixels.push_back(pixel));

            let zoomed_length = config.zoomed_length();
            let scroll_start = config.scroll_start();
            let scroll_stop = config.scroll_stop();

            let mut cropped_pixels: Vec<Color32> = Vec::with_capacity(zoomed_length * PLOT_DEPTH);
            for y in 0..PLOT_DEPTH {
                let offset = y * config.effective_len();
                for x in scroll_start..scroll_stop {
                    cropped_pixels.push(self.pixels[offset + x]);
                }
            }

            let cropped_image = ColorImage {
                size: [zoomed_length, PLOT_DEPTH],
                pixels: cropped_pixels,
            };

            self.sender.send(cropped_image).unwrap();
        }
    }
}
