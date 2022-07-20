#[derive(Copy, Clone)]
pub struct Configuration {
    pub audio_sample_rate: usize,
    pub fft_depth: usize,
    pub min_db: f32,
    pub max_db: f32,

    // Trim FFT output to this frequency. Lets you trim out unnecessary data
    // when the audio_sample_rate is much higher than needed.
    pub trim_hz: usize,

    // This actually can only be set at compile time
    // waterfall_depth: usize,
}

impl Configuration {
    // pub fn spectrum_width(&self) -> usize {
    //     self.fft_depth / 2 + 1
    // }

    // pub fn full_spectrum() -> Self {
    //     Self {
    //         audio_sample_rate: 44100,
    //         fft_depth: 4096,
    //         min_db: -20.0,
    //         max_db: 30.0,
    //         trim_hz: 44100,
    //     }
    // }

    pub fn ssb_passband() -> Self {
        Self {
            audio_sample_rate: 44100,
            fft_depth: 8192,
            min_db: -40.0,
            max_db: 0.0,
            trim_hz: 4000,
        }
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration::ssb_passband()
    }
}
