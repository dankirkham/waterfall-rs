use std::cmp::Ordering;

use crate::configuration::AudioSampleRate;
use crate::types::SampleType;
use crate::units::{Frequency, Time};
use crate::dsp::fft::Fft;
use crate::dsp::ifft::Ifft;

/// Performs coarse synchronization per "Synchronization in FT8" by Mike
/// Hasselbeck, WB2FKO
pub struct Synchronizer {
    fft_coarse: Fft,
    fft_fine: Fft,
    ifft: Ifft,
}

#[derive(Debug)]
struct Candidate {
    time: Time,
    frequency: Frequency,
    strength: f32,
}

impl Candidate {
    pub fn normalized(mut self, norm_value: f32) -> Self {
        self.strength = self.strength / norm_value;
        self
    }
}

impl PartialOrd for Candidate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Candidate {
    fn cmp(&self, other: &Self) -> Ordering {
        if other.strength > self.strength {
            Ordering::Greater
        } else if other.strength < self.strength {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

impl PartialEq for Candidate {
    fn eq(&self, other: &Self) -> bool {
        self.strength == other.strength
    }
}

impl Eq for Candidate { }

impl Synchronizer {
    pub fn new(sample_rate: AudioSampleRate) -> Self {
        let fft_coarse = Fft::new(Time::Milliseconds(160.0), sample_rate.baseband_sample_rate());
        let fft_fine = Fft::new(Time::Seconds(16.0), Frequency::Hertz(4000.0));
        let ifft = Ifft::new(1000);

        Self {
            fft_coarse,
            fft_fine,
            ifft,
        }
    }

    fn coarse_sync(&self, signal: &[SampleType]) -> Vec<Candidate> {
        // Calculate 372 individual spectra of input signal.
        // Each 160 ms time windows, offset by 1/4 symbol or 40 ms.
        let spectra: Vec<_> = (0..372)
            .map(|i| {
                let mut samples = Vec::with_capacity(self.fft_coarse.depth);

                let quarter_symbol = self.fft_coarse.depth / 4;
                let begin = i * quarter_symbol;
                let end = begin + self.fft_coarse.depth;
                samples.extend_from_slice(&signal[begin..end]);

                self.fft_coarse.run(samples)
            })
            .collect();

        // Scan for Costas array
        // - 125 start times (-2 seconds to 3 seconds)
        // - 737 frequency offsets (200 Hz to 2500 Hz)
        let pairs = (0..125)
            .map(|t| {
                (64..=800).map(move |f| (t as i64 - 50, f))
            })
            .flatten();

        let mut candidates: Vec<_> = pairs
            .map(|(t, f)| {
                let t_a = t_n(&spectra, f, t);
                let t_0a = t_0n(&spectra, t);

                let t_b = t_n(&spectra, f, t + 143);
                let t_0b = t_0n(&spectra, t + 143);

                let t_c = t_n(&spectra, f, t + 227);
                let t_0c = t_0n(&spectra, t + 227);

                let s_abc = {
                    let t = t_a + t_b + t_c;
                    let t_n = (t_0a + t_0b + t_0c - t_a - t_b - t_c) / 6.0;

                    t / t_n
                };

                let s_bc = {
                    let t = t_b + t_c;
                    let t_n = (t_0b + t_0c - t_b - t_c) / 4.0;

                    t / t_n
                };

                let strength = f32::max(s_abc, s_bc);

                Candidate {
                    frequency: self.fft_coarse.bin_to_frequency(f),
                    time: Time::Milliseconds(t as f32 * 40.0),
                    strength,
                }
            })
            .collect::<Vec<_>>();

        candidates.sort();

        let midpoint = (candidates.len() as f32 / 2.0).ceil() as usize;
        let norm_value = candidates[midpoint].strength;

        let candidates: Vec<_> = candidates
            .into_iter()
            .map(|c| c.normalized(norm_value))
            .filter(|c| c.strength >= 1.5)
            .take(200)
            .collect();

        candidates
    }

    fn process_candidate(&self, spectrum: &[SampleType], candidate: &Candidate) -> () {
        if (candidate.time.value() < 0.0) {
            eprintln!("negative times not yet implemented");
            return;
        }

        let f_begin = candidate.frequency - Frequency::Hertz(9.375);
        let f_end = candidate.frequency + Frequency::Hertz(53.125);
        let bin_begin = self.fft_fine.frequency_to_bin(f_begin);
        let bin_end = self.fft_fine.frequency_to_bin(f_end);
        dbg!(&f_begin, &f_end, &bin_begin, &bin_end);
        assert_eq!(bin_end - bin_begin, 1000);

        let spectrum = &spectrum[bin_begin..bin_end];
        assert_eq!(spectrum.len(), 1000);
        let complex_time_domain_signal = self.ifft.run(&spectrum);
        assert_eq!(complex_time_domain_signal.len(), 3200);
    }

    fn process_candidates(&self, signal: &[SampleType], candidates: &[Candidate]) -> () {
        let signal: Vec<_> = signal.into_iter().step_by(3).collect();
        let mut signal_ext: Vec<SampleType> = Vec::with_capacity(self.fft_fine.depth);
        signal_ext.extend(signal);
        let one_second_of_zeros = vec![0.0; self.fft_fine.sample_rate.value() as usize];
        signal_ext.extend(one_second_of_zeros);
        assert_eq!(signal_ext.len(), 64000);

        let spectrum = self.fft_fine.run(signal_ext);
        dbg!(&spectrum.len());

        let message: Vec<_> = candidates.iter().map(|c| self.process_candidate(&spectrum, c)).collect();
    }

    pub fn synchronize(&self, signal: &[SampleType]) -> Option<usize> {
        return None;
        let candidates = self.coarse_sync(&signal);

        let messages = self.process_candidates(&signal, &candidates);

        None
    }
}

fn t_n(spectra: &[Vec<f32>], f:usize, t: i64) -> f32 {
    if t < 0 {
        return 0.0
    }
    let t = t as usize;

    &spectra[t + 00][f + 3]
        + &spectra[t + 04][f + 1]
        + &spectra[t + 08][f + 4]
        + &spectra[t + 12][f + 0]
        + &spectra[t + 16][f + 6]
        + &spectra[t + 20][f + 5]
        + &spectra[t + 24][f + 2]
}

fn t_0n(spectra: &[Vec<f32>], t: i64) -> f32 {
    if t < 0 {
        return 0.0
    }
    let t = t as usize;

    let lowest_7 = |c: usize| (0..7)
        .map(|i| &spectra[t + c][i])
        .sum::<SampleType>();

    (0..7).map(|i| lowest_7(i * 4)).sum::<SampleType>()
}
