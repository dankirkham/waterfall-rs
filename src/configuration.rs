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

    // [1.0, 20.0]
    pub zoom: f32,

    // [0.0, 1.0]
    pub scroll: f32,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            audio_sample_rate: 44100,
            fft_depth: 8192,
            min_db: -40.0,
            max_db: 0.0,
            trim_hz: 4000,
            zoom: 1.0,
            scroll: 0.0,
        }
    }
}
