use crate::units::Frequency;

#[derive(Copy, Clone)]
pub struct TunerSettings {
    pub lower: f32,
    pub upper: f32,
    pub carrier: f32,
}

impl Default for TunerSettings {
    fn default() -> Self {
        TunerSettings {
            lower: 0.0,
            upper: 160.0,
            carrier: 2500.0,
        }
    }
}

impl TunerSettings {
    pub fn lower_absolute(&self) -> Frequency {
        Frequency::Hertz(self.carrier + self.lower)
    }

    pub fn upper_absolute(&self) -> Frequency {
        Frequency::Hertz(self.carrier + self.upper)
    }

    pub fn lower(&self) -> Frequency {
        Frequency::Hertz(self.lower)
    }

    pub fn upper(&self) -> Frequency {
        Frequency::Hertz(self.upper)
    }

    pub fn carrier(&self) -> Frequency {
        Frequency::Hertz(self.carrier)
    }
}
