pub fn first_bad_version<T: Fn(i32) -> bool>(n: i32, is_bad_version: T) -> i32 {
    let mut start = 0;
    let mut end = n;
    while start < end {
        let mid = start + (end - start) / 2;
        if is_bad_version(mid) {
            end = mid;
        } else {
            start = mid + 1;
        }
    }
    end
}
