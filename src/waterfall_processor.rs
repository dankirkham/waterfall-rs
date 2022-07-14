use std::sync::mpsc::{Sender, Receiver};
use crate::recorder::RecorderData;
use crate::plot_data::{PlotRow, PLOT_WIDTH};

const N: usize = 11025;
// const N: usize = 4;

pub struct WaterfallProcessor {
    receiver: Receiver<RecorderData>,
    sender: Sender<PlotRow>,
}

impl WaterfallProcessor {
    pub fn new(receiver: Receiver<RecorderData>, sender: Sender<PlotRow>) -> Self {
        Self { receiver, sender }
    }

    pub fn start(&self) {
        let mut data: Vec<RecorderData> = Vec::with_capacity(N);
        loop {
            let sample = self.receiver.recv().unwrap();
            data.push(sample);
            println!("Length: {}", data.len());

            if data.len() < N {
                continue;
            }

            println!("Got enough samples");

            // We have enough data to do thing.
            self.sender.send(vec![255; PLOT_WIDTH]).unwrap();
        }
    }
}
