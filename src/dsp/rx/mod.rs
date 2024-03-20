use tokio::sync::mpsc::error::TrySendError;
use tokio::sync::mpsc::Sender;
use wasm_timer::Instant;

use crate::configuration::{AudioSampleRate, Configuration};
use crate::dsp::aggregator::Aggregator;
use crate::dsp::decode::Rtty;
use crate::message::MessageSender;
use crate::statistics::Statistics;
use crate::types::SampleType;

pub struct Rx {
    plot_sender: Option<Sender<Vec<SampleType>>>,
    message_sender: Option<MessageSender>,
    sample_rate: AudioSampleRate,
    aggregator: Aggregator,
    decoder: Rtty,
}

impl Rx {
    pub fn new(config: &Configuration) -> Self {
        let sample_rate = config.audio_sample_rate;
        let aggregator = Aggregator::new(256);

        let decoder = Rtty::new(sample_rate.into());
        Self {
            plot_sender: Default::default(),
            message_sender: Default::default(),
            sample_rate,
            aggregator,
            decoder,
        }
    }

    pub fn with_plot_sender(mut self, plot_sender: Sender<Vec<SampleType>>) -> Self {
        self.plot_sender = Some(plot_sender);
        self
    }

    pub fn with_message_sender(mut self, message_sender: MessageSender) -> Self {
        self.message_sender = Some(message_sender);
        self
    }

    pub fn run(
        &mut self,
        new_samples: Vec<SampleType>,
        config: &Configuration,
        stats: &mut Statistics,
    ) {
        let sample_rate = config.audio_sample_rate;
        if sample_rate != self.sample_rate {
            // I HATE THIS
            let plot_sender = self.plot_sender.clone();
            let message_sender = self.message_sender.clone();
            let mut rx = Self::new(config);

            if let Some(sender) = plot_sender {
                rx = rx.with_plot_sender(sender);
            }

            if let Some(sender) = message_sender {
                rx = rx.with_message_sender(sender);
            }

            *self = rx;
        }

        self.aggregator.aggregate(new_samples);

        while let Some(samples) = self.aggregator.get_slice() {
            let now = Instant::now();

            let mut output_samples: Vec<f32> = Vec::new();
            for sample in samples {
                let out = self.decoder.update(sample);
                if let Some(out) = out {
                    output_samples.push(out);
                }
            }

            if let Some(sender) = &self.plot_sender {
                if let Err(err) = sender.try_send(output_samples) {
                    match err {
                        TrySendError::Full(_) => println!("Plot ui falling behind"),
                        TrySendError::Closed(_) => (),
                    }
                }
            }

            let elapsed = now.elapsed();
            stats.rx.push(elapsed);
        }
    }
}
