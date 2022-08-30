#![feature(test)]
extern crate test;

#[cfg(test)]
mod tests {
    use super::*;

    use test::Bencher;

    use rand::Rng;

    use waterfall_rs::dsp::correlator::Correlator;
    use waterfall_rs::types::SampleType;

    #[bench]
    fn bench_correlate(b: &mut Bencher) {
        let mut rng = rand::thread_rng();

        let size = 1024;
        let mut a1: Vec<SampleType> = Vec::with_capacity(size);
        for _ in 0..size {
            a1.push(rng.gen_range(0.0..2.0_f32.powf(16.0)));
        }

        let mut a2: Vec<SampleType> = Vec::with_capacity(size);
        for _ in 0..size {
            a2.push(rng.gen_range(0.0..2.0_f32.powf(16.0)));
        }

        let correlator = Correlator::new(size);

        b.iter(|| {
            // black_box(x.powf(y).powf(x));
            correlator.correlate(&a2, &a1, true);
        });
    }
}
