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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correlate_empty() {
        let a = vec![1.0];
        let b = Vec::new();
        let result = correlate(&a, &b, false);

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_correlate_empty2() {
        let a = Vec::new();
        let b = vec![1.0];
        let result = correlate(&a, &b, false);

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_correlate_single() {
        let input = vec![2.0];
        let result = correlate(&input, &input, false);

        assert_eq!(result.len(), 1);

        assert_eq!(result[0], 4.0);
    }

    #[test]
    fn test_correlate() {
        let input = vec![-1.0, 2.0, 1.0];
        let result = correlate(&input, &input, false);

        assert_eq!(result.len(), 5);

        assert_eq!(result[0], -1.0);
        assert_eq!(result[1], 0.0);
        assert_eq!(result[2], 6.0);
        assert_eq!(result[3], 0.0);
        assert_eq!(result[4], -1.0);
    }

    #[test]
    fn test_correlate2() {
        let a = vec![-3.0, 2.0, -1.0, 1.0];
        let b = vec![-1.0, 0.0, -3.0, 2.0];
        let result = correlate(&a, &b, false);

        assert_eq!(result.len(), 7);

        assert_eq!(result[0], -6.0);
        assert_eq!(result[1], 13.0);
        assert_eq!(result[2], -8.0);
        assert_eq!(result[3], 8.0);
        assert_eq!(result[4], -5.0);
        assert_eq!(result[5], 1.0);
        assert_eq!(result[6], -1.0);
    }
}
