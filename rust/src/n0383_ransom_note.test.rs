use crate::n0383_ransom_note::can_construct;

#[test]
fn test_can_construct() {
    assert!(!can_construct("a", "b"));
    assert!(!can_construct("aa", "ab"));
    assert!(can_construct("aa", "aab"));
}
