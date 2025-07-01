use crate::n0125_valid_palindrome::*;

#[test]
fn test_is_palindrome() {
    assert!(is_palindrome("A man, a plan, a canal: Panama"));
    assert!(!is_palindrome("race a car"));
    assert!(is_palindrome(" "));
}
