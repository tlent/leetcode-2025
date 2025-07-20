use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

// O(n^2)
pub fn three_sum(mut numbers: Vec<i32>) -> Vec<Vec<i32>> {
    numbers.sort();
    let mut result = vec![];
    for (i, &v) in numbers.iter().enumerate() {
        if i > 0 && v == numbers[i - 1] {
            continue;
        }
        let mut start = i + 1;
        let mut end = numbers.len() - 1;
        while start < end {
            match (v + numbers[start] + numbers[end]).cmp(&0) {
                Ordering::Less => start += 1,
                Ordering::Greater => end -= 1,
                Ordering::Equal => {
                    result.push(vec![v, numbers[start], numbers[end]]);
                    start += 1;
                    while start < end && numbers[start] == numbers[start - 1] {
                        start += 1;
                    }
                }
            }
        }
    }
    result
}

// O(n^2) with O(n) extra space for map
pub fn three_sum_map(mut numbers: Vec<i32>) -> Vec<Vec<i32>> {
    numbers.sort();
    let map: HashMap<i32, usize> = numbers.iter().copied().zip(0..).collect();
    let mut result = vec![];
    for (i, &a) in numbers.iter().enumerate() {
        if i > 0 && a == numbers[i - 1] {
            continue;
        }
        for (j, &b) in numbers.iter().enumerate().skip(i + 1) {
            if j > i + 1 && b == numbers[j - 1] {
                continue;
            }
            let c = -(a + b);
            if let Some(&k) = map.get(&c) {
                if k > j {
                    result.push(vec![a, b, c]);
                }
            }
        }
    }
    result
}

// O(n^3)
pub fn three_sum_brute_force(numbers: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = HashSet::new();
    for (i, &a) in numbers.iter().enumerate() {
        for (j, &b) in numbers.iter().enumerate() {
            for (k, &c) in numbers.iter().enumerate() {
                if i != j && i != k && j != k && a + b + c == 0 {
                    let mut triplet = vec![a, b, c];
                    triplet.sort();
                    result.insert(triplet);
                }
            }
        }
    }
    result.into_iter().collect()
}
