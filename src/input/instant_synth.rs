use rand::{thread_rng, Rng};
use tokio::sync::mpsc::Sender;

use crate::configuration::{AudioSampleRate, Configuration};
use crate::input::Source;
use crate::synth::ft8::Ft8;
use crate::synth::Samples;
use crate::types::SampleType;
use crate::units::Frequency;

pub struct InstantSynthBuilder {
    sample_rate: AudioSampleRate,
    sender: Sender<Vec<SampleType>>,
    samples_per_run: usize,
    delay: usize,
    noise: f32,
}

impl InstantSynthBuilder {
    pub fn new(sender: Sender<Vec<SampleType>>, sample_rate: AudioSampleRate) -> Self {
        Self {
            sender,
            sample_rate,
            samples_per_run: 1024,
            delay: 0,
            noise: 0.001,
        }
    }

    pub fn with_delay(mut self, delay: usize) -> Self {
        self.delay = delay;
        self
    }

    pub fn with_noise(mut self, noise: f32) -> Self {
        self.noise = noise;
        self
    }

    pub fn build(self) -> InstantSynth {
        let carrier = Frequency::Hertz(2500.0);
        let signal = Ft8::new(self.sample_rate.as_frequency(), carrier);

        InstantSynth {
            signal,
            sender: self.sender,
            samples_per_run: self.samples_per_run,
            delay: self.delay,
            noise: self.noise,
        }
    }
}

pub struct InstantSynth {
    sender: Sender<Vec<SampleType>>,
    signal: Ft8,
    samples_per_run: usize,
    delay: usize,
    noise: f32,
}

impl Source for InstantSynth {
    fn run(&mut self, config: &Configuration) {
        let mut rng = thread_rng();

        let (noise_samples, signal_samples) = if self.delay > 0 {
            if self.delay >= self.samples_per_run {
                self.delay -= self.samples_per_run;
                (self.samples_per_run, 0)
            } else {
                let delay = self.delay;
                self.delay = 0;
                (delay, self.samples_per_run - delay)
            }
        } else {
            (0, self.samples_per_run)
        };

        let mut samples: Vec<SampleType> = Vec::with_capacity(self.samples_per_run);

        (0..noise_samples).into_iter().for_each(|_| {
            let r: f32 = rng.gen();
            let noise: f32 = self.noise * r;
            samples.push(noise);
        });

        (0..signal_samples).into_iter().for_each(|_| {
            let r: f32 = rng.gen();
            let noise: f32 = self.noise * r;
            samples.push(noise + self.signal.next());
        });

        self.sender.try_send(samples).unwrap();
    }

    fn get_tx(&self) -> Sender<Vec<SampleType>> {
        self.sender.clone()
    }
}
