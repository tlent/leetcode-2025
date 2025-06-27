use std::collections::HashSet;

pub fn has_duplicate(numbers: &[i32]) -> bool {
    numbers.iter().collect::<HashSet<_>>().len() < numbers.len()
}

pub fn has_duplicate_early_return(numbers: &[i32]) -> bool {
    let mut seen = HashSet::new();
    numbers.iter().any(|number| !seen.insert(number))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_duplicate() {
        assert!(has_duplicate(&[1, 2, 3, 1]));
        assert!(!has_duplicate(&[1, 2, 3, 4]));
        assert!(has_duplicate(&[1, 1, 1, 3, 3, 4, 3, 2, 4, 2]));
    }

    #[test]
    fn test_has_duplicate_early_return() {
        assert!(has_duplicate_early_return(&[1, 2, 3, 1]));
        assert!(!has_duplicate_early_return(&[1, 2, 3, 4]));
        assert!(has_duplicate_early_return(&[1, 1, 1, 3, 3, 4, 3, 2, 4, 2]));
    }
}
