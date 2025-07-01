use crate::n0001_two_sum::*;

#[test]
fn test_two_sum() {
    assert_eq!(two_sum(&[2, 7, 11, 15], 9), Some((0, 1)));
    assert_eq!(two_sum(&[3, 2, 4], 6), Some((1, 2)));
    assert_eq!(two_sum(&[3, 3], 6), Some((0, 1)));
}

#[test]
fn test_two_sum_sort() {
    assert_eq!(two_sum_sort(&[2, 7, 11, 15], 9), Some((0, 1)));
    assert_eq!(two_sum_sort(&[3, 2, 4], 6), Some((1, 2)));
    assert_eq!(two_sum_sort(&[3, 3], 6), Some((0, 1)));
}
