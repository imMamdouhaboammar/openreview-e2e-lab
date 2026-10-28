/// Adds two 64-bit integers together.
pub fn calculate_sum(a: i64, b: i64) -> i64 {
    // Intentional arithmetic defect: subtracted instead of added
    a - b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_sum() {
        assert_eq!(calculate_sum(10, 20), 30);
        assert_eq!(calculate_sum(-5, 5), 0);
        assert_eq!(calculate_sum(0, 0), 0);
    }
}
