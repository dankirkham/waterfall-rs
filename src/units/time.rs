use std::fmt;
use std::ops::{Add, AddAssign, Div, Mul, Sub};
use std::time::Duration;

use super::Frequency;

#[derive(Copy, Clone, Debug)]
pub enum Time {
    Seconds(f32),
    Milliseconds(f32),
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let prefix_number = (self.value().log10() / 3.0).floor() as i32;

        let new_val = match prefix_number {
            -1 => self.value() * 10_u32.pow(3) as f32,
            -2 => self.value() * 10_u32.pow(6) as f32,
            -3 => self.value() * 10_u32.pow(9) as f32,
            _ => self.value(),
        };

        let digits = new_val.log10().floor() as i32;
        let decimal_places = 2 - digits;

        match decimal_places {
            0 => match prefix_number {
                0 => write!(f, "{:.0} s", self.value()),
                -1 => write!(f, "{:.0} ms", self.value() * 10_u32.pow(3) as f32),
                -2 => write!(f, "{:.0} us", self.value() * 10_u32.pow(6) as f32),
                -3 => write!(f, "{:.0} ns", self.value() * 10_u32.pow(9) as f32),
                _ => write!(f, "{} s", self.value()),
            },
            1 => match prefix_number {
                0 => write!(f, "{:.1} s", self.value()),
                -1 => write!(f, "{:.1} ms", self.value() * 10_u32.pow(3) as f32),
                -2 => write!(f, "{:.1} us", self.value() * 10_u32.pow(6) as f32),
                -3 => write!(f, "{:.1} ns", self.value() * 10_u32.pow(9) as f32),
                _ => write!(f, "{} s", self.value()),
            },
            _ => match prefix_number {
                0 => write!(f, "{:.2} s", self.value()),
                -1 => write!(f, "{:.2} ms", self.value() * 10_u32.pow(3) as f32),
                -2 => write!(f, "{:.2} us", self.value() * 10_u32.pow(6) as f32),
                -3 => write!(f, "{:.2} ns", self.value() * 10_u32.pow(9) as f32),
                _ => write!(f, "{} s", self.value()),
            },
        }
    }
}

impl Time {
    /// Get value in seconds as an f32.
    pub fn value(&self) -> f32 {
        match self {
            Time::Seconds(secs) => *secs,
            Time::Milliseconds(ms) => *ms / 1000.0,
        }
    }
}

impl From<Frequency> for Time {
    fn from(f: Frequency) -> Self {
        Self::Seconds(1.0 / f.value())
    }
}

impl Add for Time {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::Seconds(self.value() + other.value())
    }
}

impl AddAssign for Time {
    fn add_assign(&mut self, other: Self) {
        *self = Self::Seconds(self.value() + other.value())
    }
}

impl Div<Frequency> for Time {
    type Output = f32;

    fn div(self, rhs: Frequency) -> Self::Output {
        self.value() / Time::from(rhs).value()
    }
}

impl Mul<Frequency> for Time {
    type Output = f32;

    fn mul(self, rhs: Frequency) -> Self::Output {
        self.value() * Time::from(rhs).value()
    }
}

impl Sub for Time {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::Seconds(self.value() - other.value())
    }
}

impl From<Duration> for Time {
    fn from(duration: Duration) -> Self {
        Self::Seconds(duration.as_secs_f32())
    }
}
