pub trait Filter {
    fn next(&mut self, x: f32) -> f32;
}
