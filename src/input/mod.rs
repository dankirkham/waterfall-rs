mod audio;
mod example;

use tokio::sync::mpsc::Sender;

use crate::configuration::Configuration;
use crate::types::SampleType;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum InputSource {
    Audio,
    Example,
}

pub trait Source {
    fn run(&mut self, config: &Configuration);
    fn get_tx(&self) -> Sender<Vec<SampleType>>;
}

pub use audio::Audio;
pub use example::Example;
