extern crate test;

use rand::Rng;

use crate::configuration::Configuration;
use crate::recorder::RecorderData;
use crate::units::Frequency;

use super::rx::Rx;

#[cfg(test)]
mod tests {
    use super::*;
    use test::{black_box, Bencher};

    #[bench]
    fn bench_rx(b: &mut Bencher) {
        let mut rx = Rx::new();
        let mut config = Configuration::default();

        let mut rng = rand::thread_rng();
        let mut signal: Vec<RecorderData> = Vec::with_capacity(config.fft_depth);
        for _ in 0..512 {
            signal.push(rng.gen_range(0.0..2.0_f32.powf(16.0)));
        }

        b.iter(|| {
            // black_box(x.powf(y).powf(x));
            rx.run(signal.clone(), config.clone());
        });
    }
}
