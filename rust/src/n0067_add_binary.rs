pub fn add_binary(a: &str, b: &str) -> String {
    let mut result = String::new();
    let mut carry = 0;
    let mut a_iter = a.bytes().rev().peekable();
    let mut b_iter = b.bytes().rev().peekable();
    while a_iter.peek().is_some() || b_iter.peek().is_some() || carry > 0 {
        let mut sum = carry;
        if let Some(byte) = a_iter.next() {
            sum += byte - b'0';
        }
        if let Some(byte) = b_iter.next() {
            sum += byte - b'0';
        }
        carry = sum / 2;
        result.push((b'0' + sum % 2) as char);
    }
    result.chars().rev().collect()
}
