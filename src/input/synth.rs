use rand::{thread_rng, Rng};
use tokio::sync::mpsc::Sender;
use wasm_timer::Instant;

use crate::configuration::Configuration;
use crate::input::Source;
use crate::synth::ft8::Ft8;
use crate::synth::Samples;
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
}

impl Source for Synth {
    fn run(&mut self, config: &Configuration) {
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
                let mut rng = thread_rng();

                let mut samples: Vec<SampleType> = Vec::with_capacity(new_samples);
                (0..new_samples).into_iter().for_each(|_| {
                    let r: f32 = rng.gen();
                    let noise: f32 = 0.001 * r;
                    samples.push(noise + self.signal.next());
                });

                self.sender.try_send(samples).unwrap();
            }

            Some(now)
        } else {
            Some(Instant::now())
        };
    }

    fn get_tx(&self) -> Sender<Vec<SampleType>> {
        self.sender.clone()
    }
}
