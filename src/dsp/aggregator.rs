use crate::recorder::RecorderData;

pub struct Aggregator {
    buffer_len: usize,
    data: Vec<RecorderData>,
}

impl Aggregator {
    pub fn new(buffer_len: usize) -> Self {
        Self {
            buffer_len,
            data: Vec::with_capacity(buffer_len)
        }
    }

    pub fn aggregate(&mut self, new_data: Vec<RecorderData>) {
        self.data.extend(new_data);
    }

    pub fn get_slice(&mut self) -> Option<Vec<RecorderData>> {
        if self.data.len() < self.buffer_len {
            return None;
        }

        let subset = self.data[0..self.buffer_len].to_vec();

        self.data.rotate_left(self.buffer_len);
        self.data.resize(self.data.len() - self.buffer_len, 0.0);

        Some(subset)
    }
}
