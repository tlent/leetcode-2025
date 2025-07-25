pub fn search(nums: &[i32], target: i32) -> i32 {
    let last = *nums.last().unwrap();
    let mut start: usize = 0;
    let mut end: usize = nums.len();
    while start < end {
        let mid = start + (end - start) / 2;
        if nums[mid] == target {
            return mid as i32;
        } else if target <= last && nums[mid] > last {
            start = mid + 1;
        } else if target > last && nums[mid] < last {
            end = mid;
        } else if nums[mid] < target {
            start = mid + 1;
        } else {
            end = mid;
        }
    }
    -1
}
