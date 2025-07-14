pub fn can_construct(ransom_note: &str, magazine: &str) -> bool {
    let mut counts = [0; 26];
    for b in magazine.bytes() {
        let index = (b - b'a') as usize;
        counts[index] += 1;
    }
    for b in ransom_note.bytes() {
        let index = (b - b'a') as usize;
        if counts[index] == 0 {
            return false;
        }
        counts[index] -= 1;
    }
    true
}
