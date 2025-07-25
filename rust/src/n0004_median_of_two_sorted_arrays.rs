pub fn find_median<'a>(mut a: &'a [i32], mut b: &'a [i32]) -> f64 {
    if a.len() > b.len() {
        std::mem::swap(&mut a, &mut b);
    }
    let m = a.len();
    let n = b.len();
    let length = m + n;
    let half = length / 2;
    let mut start = 0;
    let mut end = m;
    loop {
        let i = start + (end - start) / 2;
        let j = half - i;

        let a_left = if i > 0 { a[i - 1] } else { i32::MIN };
        let a_right = if i < m { a[i] } else { i32::MAX };
        let b_left = if j > 0 { b[j - 1] } else { i32::MIN };
        let b_right = if j < n { b[j] } else { i32::MAX };

        if a_left <= b_right && b_left <= a_right {
            if length % 2 == 0 {
                return (a_left.max(b_left) + a_right.min(b_right)) as f64 / 2.0;
            }
            return a_right.min(b_right) as f64;
        } else if a_left > b_right {
            end = i - 1;
        } else {
            start = i + 1;
        }
    }
}
