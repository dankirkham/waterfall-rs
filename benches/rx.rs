#![feature(test)]
extern crate test;

#[cfg(test)]
mod tests {
    use super::*;

    use test::Bencher;

    use rand::Rng;

    use waterfall_rs::configuration::Configuration;
    use waterfall_rs::dsp::rx::Rx;
    use waterfall_rs::types::WaterfallProcessor;

    #[bench]
    fn bench_rx(b: &mut Bencher) {
        let mut rx = Rx::new();
        let config = Configuration::default();

        let mut rng = rand::thread_rng();
        let buffer_len = (config.audio_sample_rate as f32 / 6.25) as usize;
        let mut signal: Vec<WaterfallProcessor> = Vec::with_capacity(buffer_len);
        for _ in 0..(buffer_len + 1) {
            signal.push(rng.gen_range(0.0..2.0_f32.powf(16.0)));
        }

        b.iter(|| {
            // black_box(x.powf(y).powf(x));
            rx.run(signal.clone(), config.clone());
        });
    }
}
