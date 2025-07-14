use crate::n0409_longest_palindrome::longest_palindrome;

#[test]
fn test_longest_palindrome() {
    assert_eq!(longest_palindrome("abccccdd"), 7);
    assert_eq!(longest_palindrome("a"), 1);
}
