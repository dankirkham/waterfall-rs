use std::fmt;

pub enum Frequency {
    Hertz(f32),
    Kilohertz(f32),
    Megahertz(f32),
}

impl fmt::Display for Frequency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let prefix_number = (self.value().log10() / 3.0).floor() as u32;

        match prefix_number {
            0 => write!(f, "{:.2} Hz", self.value()),
            1 => write!(f, "{:.2} kHz", self.value() / 10_u32.pow(3) as f32),
            2 => write!(f, "{:.2} MHz", self.value() / 10_u32.pow(6) as f32),
            _ => write!(f, "{:.2} GHz", self.value() / 10_u32.pow(9) as f32),
        }
    }
}

impl Frequency {
    /// Get value in hertz as an f32.
    pub fn value(&self) -> f32 {
        match self {
            Frequency::Hertz(hz) => *hz,
            Frequency::Kilohertz(khz) => *khz / 1_000.0,
            Frequency::Megahertz(mhz) => *mhz / 1_000_000.0,
        }
    }
}
