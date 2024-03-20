use std::collections::VecDeque;
use std::iter::zip;

#[derive(Debug)]
pub struct AsymmetricFir {
    coeff: Vec<f32>,
    state: VecDeque<f32>,
}

impl AsymmetricFir {
    pub fn new(coeff: &[f32]) -> Self {
        Self {
            coeff: Vec::from(coeff),
            state: vec![f32::default(); coeff.len()].into(),
        }
    }

    pub fn update(&mut self, input: f32) -> f32 {
        self.state.pop_front();
        self.state.push_back(input);

        zip(self.coeff.iter(), self.state.iter())
            .map(|(c, x)| c * x)
            .sum()
    }
}
