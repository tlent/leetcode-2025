use crate::n0704_binary_search::*;

#[test]
fn test_binary_search() {
    assert_eq!(binary_search(&[-1, 0, 3, 5, 9, 12], 9), 4);
    assert_eq!(binary_search(&[-1, 0, 3, 5, 9, 12], 2), -1);
}
