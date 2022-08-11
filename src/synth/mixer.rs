use crate::synth::samples::Samples;

pub struct Mixer {
    a: Box<dyn Samples + Send>,
    b: Box<dyn Samples + Send>,
}

impl Mixer {
    #[allow(dead_code)]
    pub fn new(a: Box<dyn Samples + Send>, b: Box<dyn Samples + Send>) -> Self {
        Self { a, b }
    }
}

impl Samples for Mixer {
    fn next(&mut self) -> f32 {
        self.a.next() * self.b.next()
    }
}
