use std::fmt::Display;

use crate::units::Frequency;

#[derive(Copy, Clone, PartialEq)]
pub enum AudioSampleRate {
    F8000,
    F12000,
    F16000,
    F22050,
    F44100,
    F48000,
    F96000,
}

impl Display for AudioSampleRate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match *self {
            AudioSampleRate::F8000 => "8 kHz",
            AudioSampleRate::F12000 => "12 kHz",
            AudioSampleRate::F16000 => "16 kHz",
            AudioSampleRate::F22050 => "22.05 kHz",
            AudioSampleRate::F44100 => "44.1 kHz",
            AudioSampleRate::F48000 => "48 kHz",
            AudioSampleRate::F96000 => "96 kHz",
        };

        write!(f, "{}", s)
    }
}

impl AudioSampleRate {
    pub fn samples_to_skip(&self) -> usize {
        (self.as_frequency() / self.baseband_sample_rate()) as usize
    }

    pub fn baseband_sample_rate(&self) -> Frequency {
        match *self {
            AudioSampleRate::F8000 => Frequency::Hertz(100.0),
            AudioSampleRate::F12000 => Frequency::Hertz(12000.0),
            AudioSampleRate::F16000 => Frequency::Hertz(100.0),
            AudioSampleRate::F22050 => Frequency::Hertz(105.0),
            AudioSampleRate::F44100 => Frequency::Hertz(100.0),
            AudioSampleRate::F48000 => Frequency::Hertz(12000.0),
            AudioSampleRate::F96000 => Frequency::Hertz(100.0),
        }
    }

    pub fn as_frequency(&self) -> Frequency {
        self.clone().into()
    }
}

impl From<AudioSampleRate> for Frequency {
    fn from(asr: AudioSampleRate) -> Self {
        match asr {
            AudioSampleRate::F8000 => Frequency::Hertz(8000.0),
            AudioSampleRate::F12000 => Frequency::Hertz(12000.0),
            AudioSampleRate::F16000 => Frequency::Hertz(16000.0),
            AudioSampleRate::F22050 => Frequency::Hertz(22050.0),
            AudioSampleRate::F44100 => Frequency::Hertz(44100.0),
            AudioSampleRate::F48000 => Frequency::Hertz(48000.0),
            AudioSampleRate::F96000 => Frequency::Hertz(96000.0),
        }
    }
}
