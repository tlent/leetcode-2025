use crate::n0242_valid_anagram::*;

#[test]
fn test_is_anagram() {
    assert!(is_anagram("anagram", "nagaram"));
    assert!(!is_anagram("rat", "car"));
    assert!(is_anagram("listen", "silent"));
    assert!(!is_anagram("a", "ab"));
}
