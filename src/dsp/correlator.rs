use std::sync::Arc;

use float_cmp::ApproxEq;
use rustfft::{num_complex::Complex, Fft, FftPlanner};

use crate::recorder::RecorderData;

type FftNum = f32;

pub struct Correlator {
    size: usize,
    fft: Arc<dyn Fft<FftNum>>,
    ifft: Arc<dyn Fft<FftNum>>,
}

impl Correlator {
    pub fn new(input_size: usize) -> Self {
        let size = (2 * input_size) - 1;
        let mut planner = FftPlanner::<FftNum>::new();
        let fft = planner.plan_fft_forward(size);
        let ifft = planner.plan_fft_inverse(size);
        Self { fft, ifft, size }
    }

    pub fn correlate(
        &self,
        a: &[RecorderData],
        b: &[RecorderData],
        normalize: bool,
    ) -> Vec<RecorderData> {
        let mut a_complex: Vec<Complex<FftNum>> =
            a.iter().map(|&re| Complex { re, im: 0.0 }).collect();
        a_complex.resize(self.size, Complex { re: 0.0, im: 0.0 });

        let mut b_complex: Vec<Complex<FftNum>> =
            b.iter().map(|&re| Complex { re, im: 0.0 }).collect();
        b_complex.resize(self.size, Complex { re: 0.0, im: 0.0 });

        self.fft.process(&mut a_complex);
        self.fft.process(&mut b_complex);

        let b_conj = b_complex
            .into_iter()
            .map(|Complex { re, im }| Complex { re, im: -im });

        let mut r_complex: Vec<Complex<FftNum>> =
            b_conj.zip(a_complex).map(|(a, b)| a * b).collect();

        self.ifft.process(&mut r_complex);

        let r: Vec<RecorderData> = r_complex
            .into_iter()
            .map(|c| c.re) // Use only real part
            .map(|v| v / (self.size as f32)) // Normalize
            .collect();

        let mid = self.size / 2 + 1;
        let result = [&r[mid..], &r[..mid]].concat();

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_correlate_empty() {
    //     let c = Correlator::new(1);

    //     let a = vec![1.0];
    //     let b = Vec::new();
    //     let result = c.correlate(&a, &b, false);

    //     assert_eq!(result.len(), 0);
    // }

    // #[test]
    // fn test_correlate_empty2() {
    //     let c = Correlator::new(1);

    //     let a = Vec::new();
    //     let b = vec![1.0];
    //     let result = c.correlate(&a, &b, false);

    //     assert_eq!(result.len(), 0);
    // }

    // #[test]
    // fn test_correlate_single() {
    //     let c = Correlator::new(1);

    //     let input = vec![2.0];
    //     let result = c.correlate(&input, &input, false);

    //     assert_eq!(result.len(), 1);

    //     assert_eq!(result[0], 4.0);
    // }

    #[test]
    fn test_correlate() {
        let c = Correlator::new(3);

        let input = vec![-1.0, 2.0, 1.0];
        let result = c.correlate(&input, &input, false);

        assert_eq!(result.len(), 5);

        assert!((-1.0).approx_eq(&result[0], 2.0 * ::std::f32::EPSILON, 2));
        assert!((0.0).approx_eq(&result[1], 2.0 * ::std::f32::EPSILON, 2));
        assert!((6.0).approx_eq(&result[2], 2.0 * ::std::f32::EPSILON, 2));
        assert!((0.0).approx_eq(&result[3], 2.0 * ::std::f32::EPSILON, 2));
        assert!((-1.0).approx_eq(&result[4], 2.0 * ::std::f32::EPSILON, 2));
    }

    #[test]
    fn test_correlate2() {
        let c = Correlator::new(4);

        let a = vec![-3.0, 2.0, -1.0, 1.0];
        let b = vec![-1.0, 0.0, -3.0, 2.0];
        let result = c.correlate(&a, &b, false);

        assert_eq!(result.len(), 7);

        assert!((-6.0).approx_eq(&result[0], 2.0 * ::std::f32::EPSILON, 2));
        assert!((13.0).approx_eq(&result[1], 2.0 * ::std::f32::EPSILON, 2));
        assert!((-8.0).approx_eq(&result[2], 2.0 * ::std::f32::EPSILON, 2));
        assert!((8.0).approx_eq(&result[3], 2.0 * ::std::f32::EPSILON, 2));
        assert!((-5.0).approx_eq(&result[4], 2.0 * ::std::f32::EPSILON, 2));
        assert!((1.0).approx_eq(&result[5], 5.0 * ::std::f32::EPSILON, 5));
        assert!((-1.0).approx_eq(&result[6], 5.0 * ::std::f32::EPSILON, 5));
    }
}
