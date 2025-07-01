use std::collections::HashSet;

pub fn has_duplicate(numbers: &[i32]) -> bool {
    numbers.iter().collect::<HashSet<_>>().len() < numbers.len()
}

pub fn has_duplicate_early_return(numbers: &[i32]) -> bool {
    let mut seen = HashSet::new();
    numbers.iter().any(|number| !seen.insert(number))
}
