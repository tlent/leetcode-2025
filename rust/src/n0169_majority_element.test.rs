use crate::n0169_majority_element::majority_element;

#[test]
fn test_majority_element() {
    assert_eq!(majority_element(&[3, 2, 3]), 3);
    assert_eq!(majority_element(&[2, 2, 1, 1, 1, 2, 2]), 2);
}
