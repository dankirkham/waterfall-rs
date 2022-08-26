#![feature(test)]
extern crate test;

#[cfg(test)]
mod tests {
    use super::*;

    use test::Bencher;
    use std::sync::{mpsc, Arc, RwLock};

    use egui::ColorImage;
    use rand::Rng;

    use waterfall_rs::configuration::Configuration;
    use waterfall_rs::dsp::waterfall_processor::WaterfallProcessor;
    use waterfall_rs::recorder::RecorderData;

    #[bench]
    fn bench_waterfall_processor(b: &mut Bencher) {
        let config = Configuration::default();
        let safe_config = Arc::new(RwLock::new(config));
        let (image_tx, _image_rx) = mpsc::channel::<ColorImage>();
        let mut wp = WaterfallProcessor::new(safe_config, image_tx);

        let mut rng = rand::thread_rng();
        let buffer_len = config.fft_depth;
        let mut signal: Vec<RecorderData> = Vec::with_capacity(buffer_len);
        for _ in 0..(buffer_len + 1) {
            signal.push(rng.gen_range(0.0..2.0_f32.powf(16.0)));
        }

        b.iter(|| {
            // black_box(x.powf(y).powf(x));
            wp.run(signal.clone());
        });
    }
}
