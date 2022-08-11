use std::f32::consts::PI;

use crate::filter::Filter;
use crate::units::Frequency;

pub struct HighPassFilter {
    prev_x: Option<f32>,
    prev_y: f32,
    alpha: f32,
}

impl HighPassFilter {
    #[allow(dead_code)]
    pub fn new(alpha: f32) -> Self {
        Self {
            prev_x: None,
            prev_y: 0.0,
            alpha,
        }
    }

    pub fn from_frequency(cutoff: Frequency, sample_rate: Frequency) -> Self {
        let sample_period = 1.0 / sample_rate.value();
        let alpha = 1.0 / (2.0 * PI * sample_period * cutoff.value() + 1.0);
        Self {
            prev_x: None,
            prev_y: 0.0,
            alpha,
        }
    }
}

impl Filter for HighPassFilter {
    fn next(&mut self, x: f32) -> f32 {
        if let Some(prev_x) = self.prev_x {
            // y[i] := α × (y[i−1] + x[i] − x[i−1])
            let y = self.alpha * (self.prev_y + x - prev_x);
            self.prev_x = Some(x);
            self.prev_y = y;
            y
        } else {
            let y = x;
            self.prev_x = Some(x);
            self.prev_y = y;
            y
        }
    }
}
