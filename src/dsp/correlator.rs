use std::sync::Arc;

use rustfft::{num_complex::Complex, Fft, FftPlanner};

use crate::types::SampleType;

type FftNum = f32;

pub struct Correlator {
    size: usize,
    fft: Arc<dyn Fft<FftNum>>,
    ifft: Arc<dyn Fft<FftNum>>,
}

pub struct OperandData {
    fft: Vec<Complex<FftNum>>,
    sum: FftNum,
}

impl Correlator {
    pub fn new(input_size: usize) -> Self {
        let size = (2 * input_size) - 1;
        let mut planner = FftPlanner::<FftNum>::new();
        let fft = planner.plan_fft_forward(size);
        let ifft = planner.plan_fft_inverse(size);
        Self { fft, ifft, size }
    }

    pub fn prepare_lhs(&self, input: &[SampleType]) -> OperandData {
        let sum: f32 = input.iter().map(|v| v.abs().powf(2.0)).sum();

        let mut complex: Vec<Complex<FftNum>> =
            input.iter().map(|&re| Complex { re, im: 0.0 }).collect();
        complex.resize(self.size, Complex { re: 0.0, im: 0.0 });

        self.fft.process(&mut complex);

        OperandData { sum, fft: complex }
    }

    pub fn prepare_rhs(&self, input: &[SampleType]) -> OperandData {
        let data = self.prepare_lhs(input);

        let fft = data
            .fft
            .into_iter()
            .map(|Complex { re, im }| Complex { re, im: -im })
            .collect();

        OperandData { sum: data.sum, fft }
    }

    pub fn correlate(
        &self,
        a: &[SampleType],
        b: &[SampleType],
        normalize: bool,
    ) -> Vec<SampleType> {
        let lhs_data = self.prepare_lhs(a);
        let rhs_data = self.prepare_rhs(b);

        self.correlate_with_prepared(&lhs_data, &rhs_data, normalize)
    }

    pub fn correlate_with_prepared(
        &self,
        a: &OperandData,
        b: &OperandData,
        normalize: bool,
    ) -> Vec<SampleType> {
        let OperandData {
            sum: a_sum,
            fft: a_complex,
        } = a;
        let OperandData {
            sum: b_sum,
            fft: b_conj,
        } = b;

        let mut r_complex: Vec<Complex<FftNum>> =
            b_conj.iter().zip(a_complex).map(|(a, b)| a * b).collect();

        self.ifft.process(&mut r_complex);

        let r_iter = r_complex
            .into_iter()
            .map(|c| c.re) // Use only real part
            .map(|v| v / (self.size as f32)); // Normalize

        let r: Vec<SampleType> = if normalize {
            let norm = (a_sum * b_sum).sqrt();

            r_iter.map(|v: f32| v / norm).collect()
        } else {
            r_iter.map(|v| v).collect()
        };

        let mid = self.size / 2 + 1;
        [&r[mid..], &r[..mid]].concat()
    }

    pub fn correlate_max_with_prepared(
        &self,
        a: &OperandData,
        b: &OperandData,
        normalize: bool,
    ) -> SampleType {
        let OperandData {
            sum: a_sum,
            fft: a_complex,
        } = a;
        let OperandData {
            sum: b_sum,
            fft: b_conj,
        } = b;

        let mut r_complex: Vec<Complex<FftNum>> =
            b_conj.iter().zip(a_complex).map(|(a, b)| a * b).collect();

        self.ifft.process(&mut r_complex);

        let r_iter = r_complex
            .into_iter()
            .map(|c| c.re); // Use only real part

        let mut max = r_iter.fold(-f32::INFINITY, |a, b| a.max(b));

        max /= self.size as f32; // FFT normalize

        if normalize {
            let norm = (a_sum * b_sum).sqrt(); // Cross-correlation normalize
            max /= norm
        }

        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::ApproxEq;

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
