use std::f32::consts::PI;

use realfft::RealFftPlanner;

use crate::dsp::fir::AsymmetricFir;
use crate::units::Frequency;

fn low_pass_coeff(kernel_length: usize, f_c: f32) -> Vec<f32> {
    let mut coeff: Vec<_> = (0..kernel_length)
        .map(|i| {
            if i == kernel_length / 2 {
                let m = kernel_length as f32;
                let i = i as f32;
                2. * PI * f_c * (0.42 - (0.5 * ((2. * PI * i) / m)) + (0.8 * ((4. * PI * i) / m)))
            } else {
                let m = kernel_length as f32;
                let i = i as f32;
                (2. * PI * f_c * (i - m / 2.)).sin() / (i - m / 2.)
                    * (0.42 - (0.5 * ((2. * PI * i) / m)) + (0.8 * ((4. * PI * i) / m)))
            }
        })
        .collect();

    // Normalize
    let sum: f32 = coeff.iter().sum();
    coeff.iter_mut().for_each(|c| *c /= sum);

    coeff
}

fn spectrally_invert(coeff: &mut Vec<f32>) {
    coeff.iter_mut().for_each(|c| *c *= -1.);
    let center_idx = coeff.len() / 2;
    coeff[center_idx] += 1.;
    let sum: f32 = coeff.iter().sum();
    coeff.iter_mut().for_each(|c| *c /= sum);
}

fn high_pass_coeff(kernel_length: usize, f_c: f32) -> Vec<f32> {
    let mut coeff = low_pass_coeff(kernel_length, f_c);
    spectrally_invert(&mut coeff);
    coeff
}

fn band_pass_coeff(kernel_length: usize, f_c1: f32, f_c2: f32) -> Vec<f32> {
    let c1 = high_pass_coeff(kernel_length, f_c1);
    let c2 = low_pass_coeff(kernel_length, f_c2);

    let mut real_planner = RealFftPlanner::<f64>::new();

    let r2c = real_planner.plan_fft_forward(kernel_length);

    let mut c1_input = r2c.make_input_vec();
    for (idx, val) in c1.into_iter().enumerate() {
        c1_input[idx] = val as f64;
    }
    let mut c1_spectrum = r2c.make_output_vec();
    r2c.process(&mut c1_input, &mut c1_spectrum).unwrap();

    let mut c2_input = r2c.make_input_vec();
    for (idx, val) in c2.into_iter().enumerate() {
        c2_input[idx] = val as f64;
    }
    let mut c2_spectrum = r2c.make_output_vec();
    r2c.process(&mut c2_input, &mut c2_spectrum).unwrap();

    let mut multiplied = r2c.make_output_vec();
    for i in 0..c1_spectrum.len() {
        multiplied[i] = c1_spectrum[i] * c2_spectrum[i];
    }

    let c2r = real_planner.plan_fft_inverse(kernel_length);

    let mut outdata = c2r.make_output_vec();

    c2r.process(&mut multiplied, &mut outdata).unwrap();

    let mut coeff: Vec<_> = outdata.into_iter().map(|f| f as f32).collect();
    let sum: f32 = coeff.iter().sum();
    coeff.iter_mut().for_each(|c| *c /= sum);
    coeff
}

enum Mode {
    LowPass(usize, f32),
    HighPass(usize, f32),
    BandPass(usize, f32, f32),
    BandReject(usize, f32, f32),
}

pub struct FirBuilder {
    mode: Mode,
}

impl FirBuilder {
    pub fn low_pass(length: usize, input_sample_rate: Frequency, cutoff: Frequency) -> Self {
        let cutoff = cutoff.value() / input_sample_rate.value();
        assert!(cutoff < 0.5);
        Self {
            mode: Mode::LowPass(length, cutoff),
        }
    }

    pub fn high_pass(length: usize, input_sample_rate: Frequency, cutoff: Frequency) -> Self {
        let cutoff = cutoff.value() / input_sample_rate.value();
        assert!(cutoff < 0.5);
        Self {
            mode: Mode::HighPass(length, cutoff),
        }
    }

    pub fn band_pass(
        length: usize,
        input_sample_rate: Frequency,
        cutoff1: Frequency,
        cutoff2: Frequency,
    ) -> Self {
        let cutoff1 = cutoff1.value() / input_sample_rate.value();
        let cutoff2 = cutoff2.value() / input_sample_rate.value();
        assert!(cutoff1 < 0.5);
        assert!(cutoff2 < 0.5);
        assert!(cutoff1 < cutoff2);
        Self {
            mode: Mode::BandPass(length, cutoff1, cutoff2),
        }
    }

    pub fn band_reject(
        length: usize,
        input_sample_rate: Frequency,
        cutoff1: Frequency,
        cutoff2: Frequency,
    ) -> Self {
        let cutoff1 = cutoff1.value() / input_sample_rate.value();
        let cutoff2 = cutoff2.value() / input_sample_rate.value();
        assert!(cutoff1 < 0.5);
        assert!(cutoff2 < 0.5);
        assert!(cutoff1 < cutoff2);
        Self {
            mode: Mode::BandReject(length, cutoff1, cutoff2),
        }
    }

    pub fn build_asymmetric(&self) -> AsymmetricFir {
        let coeff = match self.mode {
            Mode::LowPass(m, f_c) => low_pass_coeff(m, f_c),
            Mode::HighPass(m, f_c) => high_pass_coeff(m, f_c),
            Mode::BandPass(m, f_c1, f_c2) => band_pass_coeff(m, f_c1, f_c2),
            _ => panic!(),
        };

        AsymmetricFir::new(&coeff)
    }
}
