pub fn length_of_longest_substring(s: &str) -> usize {
    let mut max_len = 0;
    let mut start = 0;
    let mut seen = [None; 256];
    for (i, b) in s.bytes().enumerate() {
        if let Some(prev) = seen[b as usize] {
            start = start.max(prev + 1);
        }
        max_len = max_len.max(i - start + 1);
        seen[b as usize] = Some(i);
    }
    max_len
}
