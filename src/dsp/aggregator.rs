use std::collections::VecDeque;
use std::mem;

use crate::types::SampleType;

pub struct Aggregator {
    buffer_len: usize,
    data: VecDeque<SampleType>,
}

impl Aggregator {
    pub fn new(buffer_len: usize) -> Self {
        Self {
            buffer_len,
            data: VecDeque::with_capacity(10 * buffer_len),
        }
    }

    pub fn aggregate(&mut self, new_data: Vec<SampleType>) {
        self.data.extend(new_data);
    }

    pub fn return_slice(&mut self, data: Vec<SampleType>) {
        let mut data_v: VecDeque<SampleType> = data.into();
        data_v.append(&mut self.data);
        mem::swap(&mut data_v, &mut self.data);
    }

    pub fn get_slice(&mut self) -> Option<Vec<SampleType>> {
        if self.data.len() < self.buffer_len {
            return None;
        }

        // let subset = self.data[0..self.buffer_len].to_vec();
        // self.data.rotate_left(self.buffer_len);
        // self.data.resize(self.data.len() - self.buffer_len, 0.0);

        let subset: Vec<SampleType> = self.data.drain(..self.buffer_len).collect();

        Some(subset)
    }

    /// Take another Aggregator's data. It will be placed at the front of the
    /// queue.
    pub fn take_data(&mut self, donor: &mut Self) {
        std::mem::swap(&mut self.data, &mut donor.data);
        if !self.data.is_empty() {
            self.data.append(&mut donor.data);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::dsp::Aggregator;

    #[test]
    fn test_aggregate() {
        let mut agg = Aggregator::new(2);

        let a = vec![1.0, 2.0];
        let b = vec![3.0, 4.0];
        let c = vec![5.0, 6.0];

        agg.aggregate(a);
        agg.aggregate(b);

        let o1 = agg.get_slice();
        assert!(o1.is_some());
        let r1 = o1.unwrap();
        assert_eq!(r1[0], 1.0);
        assert_eq!(r1[1], 2.0);

        agg.aggregate(c);

        let o2 = agg.get_slice();
        assert!(o2.is_some());
        let r2 = o2.unwrap();
        assert_eq!(r2[0], 3.0);
        assert_eq!(r2[1], 4.0);

        let o3 = agg.get_slice();
        assert!(o3.is_some());
        let r3 = o3.unwrap();
        assert_eq!(r3[0], 5.0);
        assert_eq!(r3[1], 6.0);

        let o4 = agg.get_slice();
        assert!(o4.is_none());
    }

    #[test]
    fn test_take_data_empty() {
        let mut short = Aggregator::new(2);
        let mut long = Aggregator::new(4);

        let a = vec![1.0, 2.0];
        let b = vec![3.0, 4.0];

        long.aggregate(a);
        long.aggregate(b);

        short.take_data(&mut long);

        let o1 = short.get_slice();
        assert!(o1.is_some());
        let r1 = o1.unwrap();
        assert_eq!(r1[0], 1.0);
        assert_eq!(r1[1], 2.0);

        let o2 = short.get_slice();
        assert!(o2.is_some());
        let r2 = o2.unwrap();
        assert_eq!(r2[0], 3.0);
        assert_eq!(r2[1], 4.0);
    }

    #[test]
    fn test_take_data_not_empty() {
        let mut short = Aggregator::new(2);
        let mut long = Aggregator::new(4);

        let a = vec![1.0, 2.0];
        let b = vec![3.0, 4.0];
        let c = vec![5.0, 6.0];

        long.aggregate(a);
        long.aggregate(b);
        short.aggregate(c);

        short.take_data(&mut long);

        let o1 = short.get_slice();
        assert!(o1.is_some());
        let r1 = o1.unwrap();
        assert_eq!(r1[0], 1.0);
        assert_eq!(r1[1], 2.0);

        let o2 = short.get_slice();
        assert!(o2.is_some());
        let r2 = o2.unwrap();
        assert_eq!(r2[0], 3.0);
        assert_eq!(r2[1], 4.0);

        let o3 = short.get_slice();
        assert!(o3.is_some());
        let r3 = o3.unwrap();
        assert_eq!(r3[0], 5.0);
        assert_eq!(r3[1], 6.0);
    }

    #[test]
    fn test_return_slice() {
        let mut agg = Aggregator::new(4);

        let a = vec![1.0, 2.0];
        let b = vec![3.0, 4.0];
        let c = vec![5.0, 6.0];

        agg.aggregate(a);
        agg.aggregate(b);
        agg.aggregate(c);

        let o1 = agg.get_slice();
        let mut r1 = o1.unwrap();

        let return_vec = r1.split_off(2);

        assert_eq!(r1[0], 1.0);
        assert_eq!(r1[1], 2.0);

        assert_eq!(return_vec[0], 3.0);
        assert_eq!(return_vec[1], 4.0);

        agg.return_slice(return_vec);

        let o2 = agg.get_slice();
        let r2 = o2.unwrap();

        assert_eq!(r2[0], 3.0);
        assert_eq!(r2[1], 4.0);
    }
}
