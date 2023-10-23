use std::sync::Arc;
use std::cmp::Ordering;

use realfft::RealFftPlanner;
use realfft::RealToComplex;

use crate::configuration::AudioSampleRate;
use crate::types::SampleType;
use crate::units::Time;

/// Performs coarse synchronization per "Synchronization in FT8" by Mike
/// Hasselbeck, WB2FKO
pub struct Synchronizer {
    fft: Arc<dyn RealToComplex<f32>>,
    fft_depth: usize,
    bin_hz: f32,
}

impl Synchronizer {
    pub fn new(sample_rate: AudioSampleRate) -> Self {
        let mut planner = RealFftPlanner::<f32>::new();

        // let fft_depth = (Time::Seconds(0.320) / sample_rate.baseband_sample_rate()) as usize;
        let fft_depth = (Time::Seconds(0.160) / sample_rate.baseband_sample_rate()) as usize;

        let fft = planner.plan_fft_forward(fft_depth as usize); // Hasselbeck, pg. 6
        let bin_hz = sample_rate.baseband_sample_rate().value() / fft_depth as f32;

        Self {
            fft,
            fft_depth,
            bin_hz,
        }
    }

    pub fn synchronize(&self, signal: Vec<SampleType>) -> Option<usize> {
        // Calculate 372 individual spectra of input signal.
        // Each 160 ms time windows, offset by 1/4 symbol or 40 ms.
        let spectra: Vec<_> = (0..372).map(|i| {
            let mut samples = Vec::with_capacity(self.fft_depth);

            // let quarter_symbol = self.fft_depth / 8;
            // let begin = i * quarter_symbol;
            // let end = begin + self.fft_depth / 2;
            // samples.extend_from_slice(&signal[begin..end]);
            // samples.extend(vec![0.0; self.fft_depth / 2]);

            let quarter_symbol = self.fft_depth / 4;
            let begin = i * quarter_symbol;
            let end = begin + self.fft_depth;
            samples.extend_from_slice(&signal[begin..end]);

            let mut spectrum = self.fft.make_output_vec();
            self.fft.process(&mut samples, &mut spectrum).unwrap();
            let spectrum: Vec<_> = spectrum.into_iter().map(|c| c.norm()).collect();
            spectrum
        }).collect();

        // Scan for Costas array in 125 start times
        let pairs = (0..125).map(|t| {
            // (0..176).map(move |f| {
            (80..800).map(move |f| {
                (t, f)
            })
        }).flatten();

        let mut candidates: Vec<_> = pairs.map(|(t, f)| {
            let t_a =
                &spectra[t + 00][f + (18.75 / self.bin_hz) as usize] +
                &spectra[t + 04][f + (06.25 / self.bin_hz) as usize] +
                &spectra[t + 08][f + (25.00 / self.bin_hz) as usize] +
                &spectra[t + 12][f + (00.00 / self.bin_hz) as usize] +
                &spectra[t + 16][f + (37.50 / self.bin_hz) as usize] +
                &spectra[t + 20][f + (31.25 / self.bin_hz) as usize] +
                &spectra[t + 24][f + (12.50 / self.bin_hz) as usize];

            let t_0a =
                &spectra[t + 00][0] +
                &spectra[t + 00][1] +
                &spectra[t + 00][2] +
                &spectra[t + 00][3] +
                &spectra[t + 00][4] +
                &spectra[t + 00][5] +
                &spectra[t + 00][6] +

                &spectra[t + 04][0] +
                &spectra[t + 04][1] +
                &spectra[t + 04][2] +
                &spectra[t + 04][3] +
                &spectra[t + 04][4] +
                &spectra[t + 04][5] +
                &spectra[t + 04][6] +

                &spectra[t + 08][0] +
                &spectra[t + 08][1] +
                &spectra[t + 08][2] +
                &spectra[t + 08][3] +
                &spectra[t + 08][4] +
                &spectra[t + 08][5] +
                &spectra[t + 08][6] +

                &spectra[t + 12][0] +
                &spectra[t + 12][1] +
                &spectra[t + 12][2] +
                &spectra[t + 12][3] +
                &spectra[t + 12][4] +
                &spectra[t + 12][5] +
                &spectra[t + 12][6] +

                &spectra[t + 16][0] +
                &spectra[t + 16][1] +
                &spectra[t + 16][2] +
                &spectra[t + 16][3] +
                &spectra[t + 16][4] +
                &spectra[t + 16][5] +
                &spectra[t + 16][6] +

                &spectra[t + 20][0] +
                &spectra[t + 20][1] +
                &spectra[t + 20][2] +
                &spectra[t + 20][3] +
                &spectra[t + 20][4] +
                &spectra[t + 20][5] +
                &spectra[t + 20][6] +

                &spectra[t + 24][0] +
                &spectra[t + 24][1] +
                &spectra[t + 24][2] +
                &spectra[t + 24][3] +
                &spectra[t + 24][4] +
                &spectra[t + 24][5] +
                &spectra[t + 24][6];

            (t as f32 * 0.40, f as f32 * self.bin_hz, t_a / t_0a)
        })
            .collect::<Vec<_>>();

        candidates
            .sort_by(|&a, &b| if a.2 < b.2 {
                Ordering::Greater
            } else {
                Ordering::Less
            });

        let candidates: Vec<_> = candidates
            .into_iter()
            .take(10)
            .collect();

        dbg!(candidates);

        None
    }
}
