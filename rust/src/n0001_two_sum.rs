use std::{cmp::Ordering, collections::HashMap};

pub fn two_sum(numbers: &[i32], target: i32) -> Option<(usize, usize)> {
    let mut seen: HashMap<i32, usize> = HashMap::new();
    for (index, &number) in numbers.iter().enumerate() {
        let complement = target - number;
        if let Some(&complement_index) = seen.get(&complement) {
            return Some((complement_index, index));
        }
        seen.insert(number, index);
    }
    None
}

pub fn two_sum_sort(numbers: &[i32], target: i32) -> Option<(usize, usize)> {
    let mut values: Vec<(usize, i32)> = numbers.iter().copied().enumerate().collect();
    values.sort_unstable_by_key(|&(_, number)| number);
    let mut start = 0;
    let mut end = values.len() - 1;
    while start < end {
        let (start_index, start_number) = values[start];
        let (end_index, end_number) = values[end];
        let sum = start_number + end_number;
        match sum.cmp(&target) {
            Ordering::Equal => return Some((start_index, end_index)),
            Ordering::Less => start += 1,
            Ordering::Greater => end -= 1,
        };
    }
    None
}
