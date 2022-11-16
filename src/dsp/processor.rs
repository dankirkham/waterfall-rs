use egui::ColorImage;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::configuration::Configuration;
use crate::message::MessageSender;
use crate::statistics::Statistics;
use crate::types::SampleType;

use super::rx::Rx;
use super::waterfall_processor::WaterfallProcessor;

pub struct Processor {
    receiver: Receiver<Vec<SampleType>>,
    rx: Rx,
    wp: WaterfallProcessor,
}

impl Processor {
    pub fn new(
        receiver: Receiver<Vec<SampleType>>,
        sender: Sender<ColorImage>,
        plot_sender: Sender<Vec<SampleType>>,
        message_sender: MessageSender,
        config: &Configuration,
    ) -> Self {
        let rx = Rx::new(plot_sender, message_sender, config);
        let wp = WaterfallProcessor::new(sender, config);

        Self { receiver, rx, wp }
    }

    pub fn run(&mut self, config: &Configuration, stats: &mut Statistics) {
        while let Ok(samples) = self.receiver.try_recv() {
            // use std::time::Instant;
            // let now = Instant::now();

            self.rx.run(samples.clone(), config, stats);
            self.wp.run(samples, config, stats);

            // let elapsed = now.elapsed();
            // println!("Elapsed: {:.2?}", elapsed);
        }
    }
}
