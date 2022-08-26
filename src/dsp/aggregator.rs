use std::collections::VecDeque;

use crate::recorder::RecorderData;

pub struct Aggregator {
    buffer_len: usize,
    data: VecDeque<RecorderData>,
}

impl Aggregator {
    pub fn new(buffer_len: usize) -> Self {
        Self {
            buffer_len,
            data: VecDeque::with_capacity(10 * buffer_len)
        }
    }

    pub fn aggregate(&mut self, new_data: Vec<RecorderData>) {
        self.data.extend(new_data);
    }

    pub fn get_slice(&mut self) -> Option<Vec<RecorderData>> {
        if self.data.len() < self.buffer_len {
            return None;
        }

        // let subset = self.data[0..self.buffer_len].to_vec();
        // self.data.rotate_left(self.buffer_len);
        // self.data.resize(self.data.len() - self.buffer_len, 0.0);

        let subset: Vec<RecorderData> = self.data.drain(..self.buffer_len).collect();

        Some(subset)
    }
}

mod tests {
    use super::*;

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
}
