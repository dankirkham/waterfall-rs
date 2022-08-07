use std::f32::consts::PI;

use crate::filter::filter::Filter;
use crate::units::Frequency;

pub struct LowPassFilter {
    prev_y: Option<f32>,
    alpha: f32,
}

impl LowPassFilter {
    pub fn new(alpha: f32) -> Self {
        Self {
            prev_y: None,
            alpha,
        }
    }

    pub fn from_frequency(cutoff: Frequency, sample_rate: Frequency) -> Self {
        let sample_period = 1.0 / sample_rate.value();
        let alpha = (2.0 * PI * sample_period * cutoff.value())
            / (2.0 * PI * sample_period * cutoff.value() + 1.0);

        Self {
            prev_y: None,
            alpha,
        }
    }
}

impl Filter for LowPassFilter {
    fn next(&mut self, x: f32) -> f32 {
        if let Some(prev_y) = self.prev_y {
            // y[i] := y[i-1] + α * (x[i] - y[i-1])
            let y = prev_y + self.alpha * (x - prev_y);
            self.prev_y = Some(y);
            y
        } else {
            let y = x * self.alpha;
            self.prev_y = Some(y);
            y
        }
    }
}
