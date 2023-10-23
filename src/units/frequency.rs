use std::fmt;
use std::ops::{Add, AddAssign, Div, Mul, Sub};

use crate::units::Time;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Frequency {
    Hertz(f32),
    // Kilohertz(f32),
    // Megahertz(f32),
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
            // Frequency::Kilohertz(khz) => *khz * 1_000.0,
            // Frequency::Megahertz(mhz) => *mhz * 1_000_000.0,
        }
    }
}

impl Add for Frequency {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::Hertz(self.value() + other.value())
    }
}

impl AddAssign for Frequency {
    fn add_assign(&mut self, other: Self) {
        *self = Self::Hertz(self.value() + other.value())
    }
}

impl Sub for Frequency {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::Hertz(self.value() - other.value())
    }
}

impl Div<Time> for Frequency {
    type Output = f32;

    fn div(self, rhs: Time) -> Self::Output {
        Time::from(self).value() / rhs.value()
    }
}

impl Mul<Time> for Frequency {
    type Output = f32;

    fn mul(self, rhs: Time) -> Self::Output {
        Time::from(self).value() * rhs.value()
    }
}

impl Div<Frequency> for Frequency {
    type Output = f32;

    fn div(self, rhs: Self) -> Self::Output {
        self.value() / rhs.value()
    }
}

impl Div<usize> for Frequency {
    type Output = Frequency;

    fn div(self, rhs: usize) -> Self::Output {
        Self::Hertz(((self.value() as usize) / rhs) as f32)
    }
}

impl Div<f32> for Frequency {
    type Output = Frequency;

    fn div(self, rhs: f32) -> Self::Output {
        Self::Hertz((self.value() / rhs) as f32)
    }
}
