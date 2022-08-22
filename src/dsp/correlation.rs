use rayon::prelude::*;

use crate::recorder::RecorderData;

pub fn correlate(a: &[RecorderData], b: &[RecorderData], normalize: bool) -> Vec<RecorderData> {
    let a_len: i32 = a.len().try_into().unwrap();
    let b_len: i32 = b.len().try_into().unwrap();

    let result = ((-b_len + 1)..a_len).into_par_iter().map(|p| {
        let left = p.max(0);
        let right = (b_len + p).min(a_len);

        let a_slc = &a[left as usize..right as usize];
        let b_slc = &b[(left - p) as usize..(right - p) as usize];
        let sum = a_slc.iter().zip(b_slc).map(|(&i1, &i2)| i1 * i2).sum();

        sum
    });

    if normalize {
        let a_sum: f32 = a.iter().map(|v| v.abs().powf(2.0)).sum();
        let b_sum: f32 = b.iter().map(|v| v.abs().powf(2.0)).sum();
        let norm = (a_sum * b_sum).sqrt();

        result.map(|v: f32| v / norm).collect::<Vec<RecorderData>>()
    } else {
        result.collect::<Vec<RecorderData>>()
    }
}
