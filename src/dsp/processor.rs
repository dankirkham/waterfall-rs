use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, RwLock};

use egui::ColorImage;

use crate::configuration::Configuration;
use crate::recorder::RecorderData;

use super::rx::Rx;
use super::waterfall_processor::WaterfallProcessor;

pub struct Processor {
    receiver: Receiver<Vec<RecorderData>>,
    config: Arc<RwLock<Configuration>>,
    rx: Rx,
    wp: WaterfallProcessor,
}

impl Processor {
    pub fn new(
        receiver: Receiver<Vec<RecorderData>>,
        sender: Sender<ColorImage>,
        _plot: Sender<Vec<RecorderData>>,
        config: Arc<RwLock<Configuration>>,
    ) -> Self {
        let rx = Rx::new();
        let wp = WaterfallProcessor::new(config.clone(), sender);

        Self {
            receiver,
            config,
            rx,
            wp,
        }
    }

    pub fn run(&mut self) {
        while let Ok(samples) = self.receiver.try_recv() {
            let config = *self.config.read().unwrap();

            // use std::time::Instant;
            // let now = Instant::now();

            self.rx.run(samples.clone(), config.clone());
            self.wp.run(samples);

            // let elapsed = now.elapsed();
            // println!("Elapsed: {:.2?}", elapsed);
        }
    }
}
