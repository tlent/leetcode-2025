pub fn valid_parentheses(s: &str) -> bool {
    let mut stack = vec![];
    for b in s.bytes() {
        match b {
            b'(' => stack.push(b')'),
            b'{' => stack.push(b'}'),
            b'[' => stack.push(b']'),
            _ => {
                if stack.pop() != Some(b) {
                    return false;
                }
            }
        }
    }
    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_parentheses() {
        assert!(valid_parentheses("()"));
        assert!(valid_parentheses("()[]{}"));
        assert!(!valid_parentheses("(]"));
        assert!(valid_parentheses("([])"));
    }
}
