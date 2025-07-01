use crate::n0217_contains_duplicate::*;

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
