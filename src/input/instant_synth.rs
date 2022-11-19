use rand::{thread_rng, Rng};
use tokio::sync::mpsc::Sender;

use crate::configuration::{AudioSampleRate, Configuration};
use crate::input::Source;
use crate::synth::ft8::Ft8;
use crate::synth::Samples;
use crate::types::SampleType;
use crate::units::Frequency;

pub struct InstantSynth {
    sender: Sender<Vec<SampleType>>,
    // config: Arc<RwLock<Configuration>>,
    sample_rate: AudioSampleRate,
    signal: Ft8,
}

impl InstantSynth {
    pub fn new(sender: Sender<Vec<SampleType>>, config: &Configuration) -> Self {
        let sample_rate = config.audio_sample_rate;
        let carrier = Frequency::Hertz(2500.0);
        let signal = Ft8::new(sample_rate.as_frequency(), carrier);

        Self {
            sender,
            sample_rate,
            signal,
        }
    }
}

impl Source for InstantSynth {
    fn run(&mut self, config: &Configuration) {
        let sample_rate = config.audio_sample_rate;
        if sample_rate != self.sample_rate {
            self.sample_rate = sample_rate;
            let carrier = Frequency::Hertz(2500.0);
            self.signal = Ft8::new(sample_rate.as_frequency(), carrier);
        }

        let new_samples = 1024;
        let mut rng = thread_rng();

        let mut samples: Vec<SampleType> = Vec::with_capacity(new_samples);
        (0..new_samples).into_iter().for_each(|_| {
            let r: f32 = rng.gen();
            let noise: f32 = 0.001 * r;
            samples.push(noise + self.signal.next());
        });

        self.sender.try_send(samples).unwrap();
    }

    fn get_tx(&self) -> Sender<Vec<SampleType>> {
        self.sender.clone()
    }
}
