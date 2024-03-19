use std::borrow::Cow;
use std::io::{BufReader, Cursor};

use hound::WavReader;
use rust_embed::RustEmbed;
use tokio::sync::mpsc::Sender;
use wasm_timer::Instant;

use crate::configuration::{AudioSampleRate, Configuration};
use crate::input::Source;
use crate::types::SampleType;

#[derive(RustEmbed)]
#[folder = "example_signals/"]
struct Asset;

pub struct Example<'a> {
    sender: Sender<Vec<SampleType>>,
    sample_rate: AudioSampleRate,
    last_time: Option<Instant>,
    signal: WavReader<BufReader<std::io::Cursor<Cow<'a, [u8]>>>>,
}

impl<'a> Example<'a> {
    pub fn new(sender: Sender<Vec<SampleType>>, config: &Configuration) -> Self {
        let sample_rate = config.audio_sample_rate;

        // let file = Asset::get("210703_133430.wav").unwrap();
        let file = Asset::get("RTTY_170Hz_45.45Bd.wav").unwrap();
        let cursor = Cursor::new(file.data);
        let reader = BufReader::new(cursor);
        let signal = WavReader::new(reader).unwrap();

        Self {
            sender,
            sample_rate,
            last_time: None,
            signal,
        }
    }
}

impl<'a> Source for Example<'a> {
    fn run(&mut self, config: &Configuration) {
        let sample_rate = config.audio_sample_rate;
        if sample_rate != self.sample_rate {
            panic!("hmm");
        }

        self.last_time = if let Some(last_time) = self.last_time {
            let now = Instant::now();

            let elapsed = (now - last_time).as_secs_f32();
            let new_samples = (elapsed * self.sample_rate.as_frequency().value()) as usize;

            if new_samples > 0 {
                let mut samples: Vec<SampleType> = Vec::with_capacity(new_samples);
                (0..new_samples).into_iter().for_each(|_| {
                    let sample = match self.signal.samples::<i16>().next() {
                        Some(sample) => sample.unwrap(),
                        None => {
                            self.signal.seek(0).unwrap();
                            self.signal.samples::<i16>().next().unwrap().unwrap()
                        }
                    };
                    let sample = f32::from(sample);
                    samples.push(sample / 2_f32.powf(16.));
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
