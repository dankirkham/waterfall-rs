#[cfg(test)]
mod tests {
    use waterfall_rs::dsp::correlation::correlate;

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
