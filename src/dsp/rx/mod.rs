use tokio::sync::mpsc::Sender;
use wasm_timer::Instant;

use crate::configuration::{AudioSampleRate, Configuration};
use crate::dsp::aggregator::Aggregator;
use crate::message::MessageSender;
use crate::statistics::Statistics;
use crate::types::SampleType;
use crate::units::Time;

pub struct Rx {
    plot_sender: Option<Sender<Vec<SampleType>>>,
    message_sender: Option<MessageSender>,

    sample_rate: AudioSampleRate,

    aggregator: Aggregator,
}

impl Rx {
    pub fn new(config: &Configuration) -> Self {
        let sample_rate = config.audio_sample_rate;

        let aggregator_len = (Time::Seconds(15.0) / sample_rate.as_frequency()) as usize;
        let aggregator = Aggregator::new(aggregator_len);
        Self {
            plot_sender: Default::default(),
            message_sender: Default::default(),

            sample_rate,

            aggregator,
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

        while let Some(mut _samples) = self.aggregator.get_slice() {
            let now = Instant::now();

            let elapsed = now.elapsed();
            stats.rx.push(elapsed);
        }
    }
}
