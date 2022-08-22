#![feature(test)]
extern crate test;

#[cfg(test)]
mod tests {
    use super::*;

    use test::Bencher;

    use rand::Rng;

    use waterfall_rs::dsp::correlation::correlate;
    use waterfall_rs::recorder::RecorderData;

    #[bench]
    fn bench_correlate(b: &mut Bencher) {
        let mut rng = rand::thread_rng();

        let size1 = 1024;
        let mut a1: Vec<RecorderData> = Vec::with_capacity(size1);
        for _ in 0..size1 {
            a1.push(rng.gen_range(0.0..2.0_f32.powf(16.0)));
        }

        let size2 = 1024;
        let mut a2: Vec<RecorderData> = Vec::with_capacity(size2);
        for _ in 0..size2 {
            a2.push(rng.gen_range(0.0..2.0_f32.powf(16.0)));
        }

        b.iter(|| {
            // black_box(x.powf(y).powf(x));
            correlate(&a2, &a1, true);
        });
    }
}
