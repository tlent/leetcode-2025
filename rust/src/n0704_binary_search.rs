use std::cmp::Ordering;

pub fn binary_search(numbers: &[i32], target: i32) -> i32 {
    let mut start = 0;
    let mut end = numbers.len() - 1;
    while start <= end {
        let mid = (start + end) / 2;
        match numbers[mid].cmp(&target) {
            Ordering::Less => start = mid + 1,
            Ordering::Equal => return mid as i32,
            Ordering::Greater => end = mid - 1,
        }
    }
    -1
}
