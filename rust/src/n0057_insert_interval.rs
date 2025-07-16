pub fn insert(intervals: Vec<Vec<i32>>, mut new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    let start = intervals
        .binary_search_by(|interval| {
            if interval[1] < new_interval[0] {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        })
        .unwrap_or_else(|x| x);
    let end = intervals
        .binary_search_by(|interval| {
            if interval[0] <= new_interval[1] {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        })
        .unwrap_or_else(|x| x);

    if start < end {
        new_interval[0] = new_interval[0].min(intervals[start][0]);
        new_interval[1] = new_interval[1].max(intervals[end - 1][1]);
    }

    [&intervals[..start], &[new_interval], &intervals[end..]].concat()
}
