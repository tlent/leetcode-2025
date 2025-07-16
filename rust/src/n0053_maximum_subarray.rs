pub fn max_subarray(numbers: &[i32]) -> i32 {
    let mut max = numbers[0];
    let mut sum = numbers[0];

    for &number in &numbers[1..] {
        if sum < 0 {
            sum = 0;
        }
        sum += number;
        max = max.max(sum);
    }

    max
}
