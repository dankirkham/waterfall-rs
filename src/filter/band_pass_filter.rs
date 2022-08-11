use crate::filter::Filter;
use crate::filter::high_pass_filter::HighPassFilter;
use crate::filter::low_pass_filter::LowPassFilter;
use crate::units::Frequency;

pub struct BandPassFilter {
    low: HighPassFilter,
    high: LowPassFilter,
}

impl BandPassFilter {
    #[allow(dead_code)]
    pub fn from_frequency(
        low_cutoff: Frequency,
        high_cutoff: Frequency,
        sample_rate: Frequency,
    ) -> Self {
        let low = HighPassFilter::from_frequency(low_cutoff, sample_rate);
        let high = LowPassFilter::from_frequency(high_cutoff, sample_rate);

        Self { low, high }
    }
}

impl Filter for BandPassFilter {
    fn next(&mut self, x: f32) -> f32 {
        self.low.next(self.high.next(x))
    }
}
