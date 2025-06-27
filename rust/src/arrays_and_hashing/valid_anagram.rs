pub fn is_anagram(s: &str, t: &str) -> bool {
    if s.len() != t.len() {
        return false;
    }
    let mut counts = s.bytes().fold([0; 26], |mut counts, byte| {
        counts[(byte - b'a') as usize] += 1;
        counts
    });
    for byte in t.bytes() {
        let count = &mut counts[(byte - b'a') as usize];
        if *count == 0 {
            return false;
        }
        *count -= 1;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_anagram() {
        assert!(is_anagram("anagram", "nagaram"));
        assert!(!is_anagram("rat", "car"));
        assert!(is_anagram("listen", "silent"));
        assert!(!is_anagram("a", "ab"));
    }
}
