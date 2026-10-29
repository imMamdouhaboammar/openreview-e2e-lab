/// Adds two 64-bit integers together, returning `None` if overflow or underflow occurred.
pub fn calculate_sum(a: i64, b: i64) -> Option<i64> {
    a.checked_add(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_sum() {
        assert_eq!(calculate_sum(10, 20), Some(30));
        assert_eq!(calculate_sum(-5, 5), Some(0));
        assert_eq!(calculate_sum(0, 0), Some(0));
    }

    #[test]
    fn test_calculate_sum_overflow_regression() {
        assert_eq!(calculate_sum(i64::MAX, 1), None);
        assert_eq!(calculate_sum(i64::MAX, 0), Some(i64::MAX));
        assert_eq!(calculate_sum(i64::MAX, -1), Some(i64::MAX - 1));
        assert_eq!(calculate_sum(i64::MAX, i64::MAX), None);

        assert_eq!(calculate_sum(i64::MIN, -1), None);
        assert_eq!(calculate_sum(i64::MIN, 0), Some(i64::MIN));
        assert_eq!(calculate_sum(i64::MIN, 1), Some(i64::MIN + 1));
        assert_eq!(calculate_sum(i64::MIN, i64::MIN), None);
    }
}
