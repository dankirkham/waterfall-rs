mod conditioner;
mod synchronizer;
mod symbolizer;
mod rx_mode;

use tokio::sync::mpsc::error::TrySendError;
use tokio::sync::mpsc::Sender;
use wasm_timer::Instant;

use conditioner::Conditioner;
use rx_mode::RxMode;
use synchronizer::Synchronizer;
use symbolizer::Symbolizer;

use crate::configuration::Configuration;
use crate::dsp::aggregator::Aggregator;
use crate::message::{Ft8Message, MessageSender};
use crate::statistics::Statistics;
use crate::types::SampleType;
use crate::units::Frequency;

pub struct Rx {
    aggregator: Aggregator,
    sync_aggregator: Aggregator,
    sample_rate: Frequency,
    plot_sender: Sender<Vec<SampleType>>,
    message_sender: MessageSender,
    downsample_skip: usize,
    mode: RxMode,
    conditioner: Conditioner,
    synchronizer: Synchronizer,
    symbolizer: Symbolizer,
}

impl Rx {
    pub fn new(
        plot_sender: Sender<Vec<SampleType>>,
        message_sender: MessageSender,
        config: &Configuration,
    ) -> Self {
        let sample_rate_raw = config.audio_sample_rate;
        let sample_rate = Frequency::Hertz(sample_rate_raw as f32);
        let baseband_sample_rate = match sample_rate_raw {
            8000 => Frequency::Hertz(100.0),
            16000 => Frequency::Hertz(100.0),
            22050 => Frequency::Hertz(105.0),
            44100 => Frequency::Hertz(100.0),
            48000 => Frequency::Hertz(1000.0),
            96000 => Frequency::Hertz(100.0),
            _ => sample_rate,
        };

        let downsample_skip: usize = sample_rate_raw / (baseband_sample_rate.value() as usize);
        let conditioner = Conditioner::new()
            .with_downsample_skip(downsample_skip);

        let aggregator_len = (sample_rate.value() / 6.25) as usize;
        let aggregator = Aggregator::new(aggregator_len);
        let sync_aggregator = Aggregator::new(aggregator_len * 7 * 2);

        let buffer_len = (baseband_sample_rate.value() / 6.25) as usize;

        let synchronizer = Synchronizer::new(buffer_len, baseband_sample_rate)
            .with_downsample_skip(downsample_skip);

        let symbolizer = Symbolizer::new(buffer_len, baseband_sample_rate);

        Self {
            aggregator,
            sync_aggregator,
            sample_rate,
            plot_sender,
            message_sender,
            downsample_skip,
            mode: RxMode::default(),
            conditioner,
            synchronizer,
            symbolizer,
        }
    }

    pub fn run(
        &mut self,
        new_samples: Vec<SampleType>,
        config: &Configuration,
        stats: &mut Statistics,
    ) {
        let sample_rate = Frequency::Hertz(config.audio_sample_rate as f32);
        if sample_rate.value() != self.sample_rate.value() {
            *self = Self::new(
                self.plot_sender.clone(),
                self.message_sender.clone(),
                config,
            );
        }

        let (aggregator, other_aggregator) = match self.mode.is_sync() {
            true => (&mut self.sync_aggregator, &mut self.aggregator),
            false => (&mut self.aggregator, &mut self.sync_aggregator),
        };
        aggregator.aggregate(new_samples);

        while let Some(mut samples) = aggregator.get_slice() {
            let now = Instant::now();

            let signal = self.conditioner.condition(&config, &samples);

            if let Err(err) = self.plot_sender.try_send(signal.clone()) {
                match err {
                    TrySendError::Full(_) => println!("Plot ui falling behind"),
                    TrySendError::Closed(_) => (),
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
                println!("Message received");
                let message = Box::new(Ft8Message::new());
                if let Err(err) = self.message_sender.try_send(message) {
                    match err {
                        TrySendError::Full(_) => println!("Message receiver falling behind."),
                        TrySendError::Closed(_) => (),
                    }
                }
                self.mode.reset();
            }

            let elapsed = now.elapsed();
            stats.rx.push(elapsed);
        }
    }
}
