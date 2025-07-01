use crate::n0020_valid_parentheses::*;

#[test]
fn test_valid_parentheses() {
    assert!(valid_parentheses("()"));
    assert!(valid_parentheses("()[]{}"));
    assert!(!valid_parentheses("(]"));
    assert!(valid_parentheses("([])"));
}
