mod band_pass_filter;
mod high_pass_filter;
mod low_pass_filter;

pub trait Filter {
    fn next(&mut self, x: f32) -> f32;
}

pub use band_pass_filter::BandPassFilter;
pub use high_pass_filter::HighPassFilter;
pub use low_pass_filter::LowPassFilter;
