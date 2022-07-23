use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, RwLock};

use egui::{Color32, ColorImage};
use realfft::RealFftPlanner;
use realfft::RealToComplex;
use rustfft::num_complex::Complex;

use crate::configuration::Configuration;
use crate::plot_data::PLOT_DEPTH;
use crate::recorder::RecorderData;
use crate::turbo::get_color;

pub struct WaterfallProcessor {
    receiver: Receiver<RecorderData>,
    sender: Sender<ColorImage>,
    fft: Arc<dyn RealToComplex<f32>>,
    config: Arc<RwLock<Configuration>>,
    fft_depth: usize,
    image: Option<ColorImage>,
    scroll: f32,
}

impl WaterfallProcessor {
    pub fn new(
        receiver: Receiver<RecorderData>,
        sender: Sender<ColorImage>,
        config: Arc<RwLock<Configuration>>,
    ) -> Self {
        let mut planner = RealFftPlanner::<f32>::new();
        let fft_depth = config.read().unwrap().fft_depth;
        let scroll = config.read().unwrap().scroll;
        let fft = planner.plan_fft_forward(fft_depth);

        Self {
            receiver,
            sender,
            fft,
            fft_depth,
            config,
            image: None,
            scroll,
        }
    }

    pub fn start(&mut self) {
        let fft_depth = self.config.read().unwrap().fft_depth;
        let mut data: Vec<RecorderData> = Vec::with_capacity(fft_depth);
        loop {
            if data.len() < self.fft_depth {
                let sample = self.receiver.recv().unwrap();
                data.push(sample);
                continue;
            } else if data.len() > self.fft_depth {
                println!("Dropping samples because of resize");
                data.resize(self.fft_depth, 0.0);
            }

            let Configuration {
                audio_sample_rate,
                fft_depth,
                min_db,
                max_db,
                trim_hz,
                zoom,
                scroll,
            } = *self.config.read().unwrap();

            if self.fft_depth != fft_depth {
                let mut planner = RealFftPlanner::<f32>::new();
                self.fft_depth = fft_depth;
                self.fft = planner.plan_fft_forward(self.fft_depth);
                continue;
            }

            // Bins are now Fs / N wide
            // Drop bins that are out of SSB passband
            let new_length =
                (trim_hz as f32 / (audio_sample_rate as f32 / fft_depth as f32)) as usize;

            if let Some(image) = &self.image {
                if image.size[0] != new_length {
                    let image = ColorImage::new([new_length, PLOT_DEPTH], Color32::default());
                    self.scroll = scroll;
                    self.image = Some(image);
                }
            } else {
                let image = ColorImage::new([new_length, PLOT_DEPTH], Color32::default());
                self.scroll = scroll;
                self.image = Some(image);
            }

            let mut spectrum = self.fft.make_output_vec();
            self.fft
                .process(data.as_mut_slice(), &mut spectrum)
                .unwrap();

            if new_length < fft_depth {
                spectrum.resize(new_length, Complex::default());
            }

            let m = 255.0 / (max_db - min_db);
            let scale_func = |x| m * (x - min_db);

            let image = self.image.as_mut().unwrap();
            image.pixels.rotate_left(new_length);

            let new_pixels = spectrum
                .iter()
                .map(|c| c.norm()) // Magnitude
                .map(|f| f / (fft_depth as f32).sqrt()) // Normalization
                .map(|f| 10.0 * f.log10()) // dB
                .map(scale_func)
                .map(|f| f.clamp(0.0, 255.0))
                .map(|f| f as usize)
                .map(|u| get_color(u))
                .map(|[r, g, b]| Color32::from_rgb(r, g, b));

            let start_offset = image.pixels.len() - new_length;
            for (i, pixel) in new_pixels.enumerate() {
                image.pixels[start_offset + i] = pixel;
            }

            let zoomed_length = ((new_length as f32) / zoom) as usize;
            let scroll_start = ((new_length - zoomed_length) as f32 * scroll) as usize;
            let scroll_stop = scroll_start + zoomed_length;

            let mut cropped_pixels: Vec<Color32> = Vec::with_capacity(zoomed_length * PLOT_DEPTH);
            for y in 0..PLOT_DEPTH {
                let offset = y * new_length;
                for x in scroll_start..scroll_stop {
                    cropped_pixels.push(image.pixels[offset + x]);
                }
            }

            let cropped_image = ColorImage {
                size: [zoomed_length, PLOT_DEPTH],
                pixels: cropped_pixels,
            };

            self.sender.send(cropped_image).unwrap();

            data.clear();
        }
    }
}
