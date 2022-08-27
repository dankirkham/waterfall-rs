use crate::units::Frequency;

#[derive(Clone, Debug, PartialEq)]
pub enum DecoderType {
    Ft8
}

#[derive(Clone)]
pub struct TunerSettings {
    pub lower: f32,
    pub upper: f32,
    pub carrier: f32,
    pub decoder: DecoderType,
}

impl Default for TunerSettings {
    fn default() -> Self {
        TunerSettings {
            lower: 0.0,
            upper: 50.0,
            carrier: 2500.0,
            decoder: DecoderType::Ft8,
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
