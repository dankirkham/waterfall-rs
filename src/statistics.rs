use std::collections::VecDeque;
use std::iter::Sum;
use std::ops::Div;
use std::time::Duration;

#[derive(Default)]
pub struct Statistics {
    pub rx: DataSeries<Duration>,
    pub render: DataSeries<Duration>,
    pub waterfall: DataSeries<Duration>,
}

pub struct DataSeries<T> {
    times: VecDeque<T>,
}

impl<T> Default for DataSeries<T> {
    fn default() -> Self {
        let times = VecDeque::with_capacity(10);

        Self { times }
    }
}

impl<T> DataSeries<T>
where
    T: Clone + Sum<T> + Div<u32, Output = T>,
{
    pub fn push(&mut self, val: T) {
        if self.times.len() >= 10 {
            self.times.pop_front();
        }

        self.times.push_back(val);
    }

    pub fn avg(&self) -> Option<T> {
        if self.times.is_empty() {
            return None;
        }

        let sum: T = self.times.clone().into_iter().sum();
        let avg = sum / self.times.len().try_into().unwrap();
        Some(avg)
    }
}
