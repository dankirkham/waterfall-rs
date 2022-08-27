use std::sync::mpsc::Sender;
use wasm_timer::Instant;

use crate::configuration::Configuration;
use crate::synth::{Ft8, Samples};
use crate::types::SampleType;
use crate::units::Frequency;

pub struct Synth {
    sender: Sender<Vec<SampleType>>,
    // config: Arc<RwLock<Configuration>>,
    sample_rate: Frequency,
    signal: Ft8,
    last_time: Option<Instant>,
}

impl Synth {
    pub fn new(sender: Sender<Vec<SampleType>>, config: &Configuration) -> Self {
        let sample_rate = Frequency::Hertz(config.audio_sample_rate as f32);
        let carrier = Frequency::Hertz(2500.0);
        let signal = Ft8::new(sample_rate, carrier);

        Self {
            sender,
            sample_rate,
            signal,
            last_time: None,
        }
    }

    pub fn run(&mut self, config: &Configuration) {
        let sample_rate = Frequency::Hertz(config.audio_sample_rate as f32);
        if sample_rate.value() != self.sample_rate.value() {
            self.sample_rate = Frequency::Hertz(config.audio_sample_rate as f32);
            let carrier = Frequency::Hertz(2500.0);
            self.signal = Ft8::new(sample_rate, carrier);
        }

        self.last_time = if let Some(last_time) = self.last_time {
            let now = Instant::now();

            let elapsed = (now - last_time).as_secs_f32();
            let new_samples = (elapsed * self.sample_rate.value()) as usize;

            if new_samples > 0 {
                let mut samples: Vec<SampleType> = Vec::with_capacity(new_samples);
                (0..new_samples).into_iter().for_each(|_| samples.push(self.signal.next()));
                self.sender.send(samples);
            }

            Some(now)
        } else {
            Some(Instant::now())
        };
    }
}
