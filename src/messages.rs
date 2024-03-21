use crate::configuration::Configuration;
use crate::message::{Message, MessageReceiver};

pub struct MessageCollector {
    rx: MessageReceiver,
    pub data: Vec<Box<dyn Message>>,
}

impl MessageCollector {
    pub fn new(rx: MessageReceiver) -> Self {
        Self {
            rx,
            data: Vec::new(),
        }
    }

    pub fn run(&mut self, _config: &mut Configuration) {
        while let Ok(data) = self.rx.try_recv() {
            self.data.push(data);
        }
    }
}
