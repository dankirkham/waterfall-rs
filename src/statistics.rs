use std::collections::VecDeque;
use std::time::Duration;

#[derive(Default)]
pub struct Statistics {
    pub rx: MovingAverage,
    pub render: MovingAverage,
    pub waterfall: MovingAverage,
}

pub struct MovingAverage {
    times: VecDeque<Duration>,
}

impl Default for MovingAverage {
    fn default() -> Self {
        let times = VecDeque::with_capacity(10);

        Self { times }
    }
}

impl MovingAverage {
    pub fn push(&mut self, duration: Duration) {
        if self.times.len() >= 10 {
            self.times.pop_front();
        }

        self.times.push_back(duration);
    }

    pub fn avg(&self) -> Option<Duration> {
        if self.times.len() == 0 {
            return None;
        }

        let sum: Duration = self.times.iter().sum();
        let avg = sum / self.times.len().try_into().unwrap();
        Some(avg)
    }
}
