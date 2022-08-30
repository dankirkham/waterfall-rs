#![feature(test)]
extern crate test;

#[cfg(test)]
mod tests {
    use super::*;

    use test::Bencher;

    use rand::Rng;
    use tokio::sync::mpsc;

    use waterfall_rs::configuration::Configuration;
    use waterfall_rs::dsp::rx::Rx;
    use waterfall_rs::types::SampleType;

    #[bench]
    fn bench_rx(b: &mut Bencher) {
        let (plot_tx, _plot_rx) = mpsc::channel::<Vec<SampleType>>(1);
        let config = Configuration::default();
        let mut rx = Rx::new(plot_tx, &config);

        let mut rng = rand::thread_rng();
        let buffer_len = (config.audio_sample_rate as f32 / 6.25) as usize;
        let mut signal: Vec<SampleType> = Vec::with_capacity(buffer_len);
        for _ in 0..(buffer_len + 1) {
            signal.push(rng.gen_range(0.0..2.0_f32.powf(16.0)));
        }

        b.iter(|| {
            // black_box(x.powf(y).powf(x));
            rx.run(signal.clone(), &config);
        });
    }
}
