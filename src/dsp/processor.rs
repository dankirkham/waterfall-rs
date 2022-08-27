use std::sync::mpsc::{Receiver, Sender};

use egui::ColorImage;

use crate::configuration::Configuration;
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
        _plot: Sender<Vec<SampleType>>,
        config: &Configuration,
    ) -> Self {
        let rx = Rx::new(config);
        let wp = WaterfallProcessor::new(sender, config);

        Self {
            receiver,
            rx,
            wp,
        }
    }

    pub fn run(&mut self, config: &Configuration) {
        while let Ok(samples) = self.receiver.try_recv() {
            // use std::time::Instant;
            // let now = Instant::now();

            self.rx.run(samples.clone(), config);
            self.wp.run(samples, config);

            // let elapsed = now.elapsed();
            // println!("Elapsed: {:.2?}", elapsed);
        }
    }
}
