mod audio_sample_rate;
mod scope_settings;
mod tuner_settings;

use crate::input::InputSource;
use crate::units::Frequency;
pub use audio_sample_rate::AudioSampleRate;
pub use scope_settings::{AxisMode, ScopeMode, ScopeSettings, TriggerMode, TriggerSettings};
pub use tuner_settings::{DecoderType, TunerSettings};

#[derive(Clone)]
pub struct Configuration {
    pub input_source: InputSource,
    pub input_device: String,
    pub audio_sample_rate: AudioSampleRate,
    pub fft_depth: usize,
    pub min_db: f32,
    pub max_db: f32,

    // Trim FFT output to this frequency. Lets you trim out unnecessary data
    // when the audio_sample_rate is much higher than needed.
    pub trim_hz: usize,

    // This actually can only be set at compile time
    // waterfall_depth: usize,

    // [1.0, 20.0]
    pub zoom: f32,

    // [0.0, 1.0]
    pub scroll: f32,

    pub tuner: TunerSettings,

    pub scope: ScopeSettings,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            input_source: InputSource::Example,
            input_device: "Default".to_owned(),
            audio_sample_rate: AudioSampleRate::F44100,
            fft_depth: 2048,
            min_db: -20.0,
            max_db: 5.0,
            trim_hz: 8000,
            zoom: 1.0,
            scroll: 0.0,
            tuner: TunerSettings::default(),
            scope: ScopeSettings::default(),
        }
    }
}

impl Configuration {
    pub fn displayed_bandwidth(&self) -> f32 {
        self.effective_trim_hz() as f32 / self.zoom
    }

    pub fn start_hz(&self) -> f32 {
        ((self.effective_trim_hz() as f32) - self.displayed_bandwidth()) * self.scroll
    }

    pub fn bin_hz(&self) -> f32 {
        let f = self.audio_sample_rate.as_frequency() / self.fft_depth as f32;
        f.value()
    }

    /// We can't do better than Nyquist.
    pub fn effective_trim_hz(&self) -> usize {
        let f = self.audio_sample_rate.as_frequency() / 2;
        let best_possible = f.value() as usize;

        self.trim_hz.min(best_possible)
    }

    pub fn effective_len(&self) -> usize {
        (self.effective_trim_hz() as f32 / self.bin_hz()) as usize
    }

    pub fn zoomed_length(&self) -> usize {
        ((self.effective_len() as f32) / self.zoom) as usize
    }

    pub fn scroll_start(&self) -> usize {
        ((self.effective_len() - self.zoomed_length()) as f32 * self.scroll) as usize
    }

    pub fn scroll_stop(&self) -> usize {
        self.scroll_start() + self.zoomed_length()
    }

    pub fn zoomed_interval_to_hz(&self, interval: f32) -> Frequency {
        let offset_bins = self.zoomed_length() as f32 * interval;
        let bin = self.scroll_start() as f32 + offset_bins;
        Frequency::Hertz(bin * self.bin_hz())
    }

    pub fn freq_to_zoom_interval(&self, f: Frequency) -> f32 {
        let bin = (f.value() / self.bin_hz()) as i32;
        let result = (bin - self.scroll_start() as i32) as f32 / self.zoomed_length() as f32;
        result.clamp(0.0, 1.0)
    }
}
