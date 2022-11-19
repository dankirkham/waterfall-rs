mod conditioner;
mod rx_mode;
mod symbolizer;
mod synchronizer;

use tokio::sync::mpsc::error::TrySendError;
use tokio::sync::mpsc::Sender;
use wasm_timer::Instant;

use conditioner::Conditioner;
use rx_mode::RxMode;
use symbolizer::Symbolizer;
use synchronizer::Synchronizer;

use crate::configuration::{AudioSampleRate, Configuration};
use crate::dsp::aggregator::Aggregator;
use crate::message::{Ft8Message, MessageSender};
use crate::statistics::Statistics;
use crate::types::SampleType;
use crate::units::Frequency;

pub struct Rx {
    plot_sender: Option<Sender<Vec<SampleType>>>,
    message_sender: Option<MessageSender>,
    mode: RxMode,

    sample_rate: AudioSampleRate,

    aggregator: Aggregator,
    sync_aggregator: Aggregator,
    conditioner: Conditioner,
    synchronizer: Synchronizer,
    symbolizer: Symbolizer,
}

impl Rx {
    pub fn new(config: &Configuration) -> Self {
        let sample_rate = config.audio_sample_rate;

        let conditioner = Conditioner::new();

        let deviation = Frequency::Hertz(6.25);
        let aggregator_len = (sample_rate.as_frequency() / deviation) as usize;
        let aggregator = Aggregator::new(aggregator_len);
        let sync_aggregator = Aggregator::new(aggregator_len * 7 * 2);

        let buffer_len = (sample_rate.baseband_sample_rate() / deviation) as usize;

        let synchronizer = Synchronizer::new(buffer_len, sample_rate);

        let symbolizer = Symbolizer::new(buffer_len, sample_rate);

        Self {
            aggregator,
            sync_aggregator,
            sample_rate,
            plot_sender: None,
            message_sender: None,
            mode: RxMode::default(),
            conditioner,
            synchronizer,
            symbolizer,
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

        let (aggregator, other_aggregator) = match self.mode.is_sync() {
            true => (&mut self.sync_aggregator, &mut self.aggregator),
            false => (&mut self.aggregator, &mut self.sync_aggregator),
        };
        aggregator.aggregate(new_samples);

        while let Some(mut samples) = aggregator.get_slice() {
            let now = Instant::now();

            let signal = self.conditioner.condition(&config, &samples);

            if let Some(sender) = &self.plot_sender {
                if let Err(err) = sender.try_send(signal.clone()) {
                    match err {
                        TrySendError::Full(_) => println!("Plot ui falling behind"),
                        TrySendError::Closed(_) => (),
                    }
                }
            }

            // Correlate
            if self.mode.is_sync() {
                if let Some(samples_to_skip) = self.synchronizer.synchronize(signal) {
                    let return_samples = samples.split_off(samples_to_skip);
                    aggregator.return_slice(return_samples);
                    other_aggregator.take_data(aggregator);
                    self.mode = self.mode.advance();
                } else {
                    self.mode = self.mode.reset();
                }
            } else {
                let _symbol = self.symbolizer.symbolize(signal);
                self.mode = self.mode.advance();
            }

            if self.mode.is_done() {
                let message = Box::new(Ft8Message::new());
                if let Some(sender) = &self.message_sender {
                    if let Err(err) = sender.try_send(message) {
                        match err {
                            TrySendError::Full(_) => println!("Message receiver falling behind."),
                            TrySendError::Closed(_) => (),
                        }
                    }
                }
                self.mode.reset();
            }

            let elapsed = now.elapsed();
            stats.rx.push(elapsed);
        }
    }
}
