pub fn majority_element(numbers: &[i32]) -> i32 {
    let mut element = 0;
    let mut count = 0;
    for &number in numbers {
        if count == 0 {
            element = number;
        }
        count += if element == number { 1 } else { -1 };
    }
    element
}
