mod audio;
mod instant_synth;
mod synth;
mod example;

use tokio::sync::mpsc::Sender;

use crate::configuration::Configuration;
use crate::types::SampleType;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum InputSource {
    Audio,
    Synth,
    Example,
}

pub trait Source {
    fn run(&mut self, config: &Configuration);
    fn get_tx(&self) -> Sender<Vec<SampleType>>;
}

pub use audio::Audio;
pub use instant_synth::{InstantSynth, InstantSynthBuilder};
pub use synth::Synth;
pub use example::Example;
