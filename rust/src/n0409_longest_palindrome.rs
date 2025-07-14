pub fn longest_palindrome(s: &str) -> i32 {
    let mut counts = [0; 52];
    for b in s.bytes() {
        let index = to_index(b);
        counts[index] += 1;
    }
    let mut length = 0;
    for count in counts {
        if count % 2 == 0 {
            length += count;
        } else {
            length += count - 1;
        }
    }
    if length < s.len() as i32 {
        length += 1;
    }
    length
}

fn to_index(b: u8) -> usize {
    (match b {
        b'a'..=b'z' => b - b'a',
        b'A'..=b'Z' => 26 + b - b'A',
        _ => panic!("invalid char"),
    }) as usize
}
