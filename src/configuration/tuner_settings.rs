use crate::units::Frequency;

#[derive(Copy, Clone)]
pub struct TunerSettings {
    pub lower: Frequency,
    pub upper: Frequency,
    pub carrier: Frequency,
}

impl Default for TunerSettings {
    fn default() -> Self {
        TunerSettings {
            lower: Frequency::Hertz(0.0),
            upper: Frequency::Hertz(160.0),
            carrier: Frequency::Hertz(2500.0),
        }
    }
}

impl TunerSettings {
    pub fn lower_absolute(&self) -> Frequency {
        self.carrier + self.lower
    }

    pub fn upper_absolute(&self) -> Frequency {
        self.carrier + self.upper
    }
}
