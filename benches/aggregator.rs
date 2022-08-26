#![feature(test)]
extern crate test;

#[cfg(test)]
mod tests {
    use super::*;

    use test::Bencher;

    use rand::Rng;

    use waterfall_rs::dsp::Aggregator;
    use waterfall_rs::recorder::RecorderData;

    #[bench]
    fn bench_aggregator(b: &mut Bencher) {
        let buffer_len = 512;

        let mut aggregator = Aggregator::new(buffer_len);
        let mut signal: Vec<RecorderData> = Vec::with_capacity(buffer_len);
        let mut rng = rand::thread_rng();
        for _ in 0..(buffer_len + 1) {
            signal.push(rng.gen_range(0.0..2.0_f32.powf(16.0)));
        }

        b.iter(|| {
            aggregator.aggregate(signal.clone());
            aggregator.get_slice();
        });
    }
}
