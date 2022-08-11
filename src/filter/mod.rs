pub mod band_pass_filter;
pub mod high_pass_filter;
pub mod low_pass_filter;

pub trait Filter {
    fn next(&mut self, x: f32) -> f32;
}
