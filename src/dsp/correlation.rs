use crate::recorder::RecorderData;

pub fn correlate(a: &[RecorderData], b: &[RecorderData]) -> Vec<RecorderData> {
    let a_len: i32 = a.len().try_into().unwrap();
    let b_len: i32 = b.len().try_into().unwrap();
    let mut result: Vec<RecorderData> = Vec::with_capacity(a.len() + b.len() - 1);
    for p in (-b_len + 1)..a_len {
        let left =  p.max(0);
        let right = (b_len + p).min(a_len);
        let mut sum = 0.0;
        dbg!(p);
        for a_idx in left..right {
            let b_idx = a_idx - p;
            let res = a[a_idx as usize] * b[b_idx as usize];
            println!("a[{}] * b[{}] = {}", a_idx, b_idx, res);
            sum += res;
        }
        result.push(sum);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correlate_empty() {
        let a = vec![1.0];
        let b = Vec::new();
        let result = correlate(&a, &b);

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_correlate_empty2() {
        let a = Vec::new();
        let b = vec![1.0];
        let result = correlate(&a, &b);

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_correlate_single() {
        let input = vec![2.0];
        let result = correlate(&input, &input);

        assert_eq!(result.len(), 1);

        assert_eq!(result[0], 4.0);
    }

    #[test]
    fn test_correlate() {
        let input = vec![-1.0, 2.0, 1.0];
        let result = correlate(&input, &input);

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
        let result = correlate(&a, &b);

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
