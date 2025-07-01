pub fn is_palindrome(s: &str) -> bool {
    let bytes = s.as_bytes();
    let mut start = 0;
    let mut end = bytes.len() - 1;
    while start < end {
        if !bytes[start].is_ascii_alphanumeric() {
            start += 1;
        } else if !bytes[end].is_ascii_alphanumeric() {
            end -= 1;
        } else if !bytes[start].eq_ignore_ascii_case(&bytes[end]) {
            return false;
        } else {
            start += 1;
            end -= 1;
        }
    }
    true
}
