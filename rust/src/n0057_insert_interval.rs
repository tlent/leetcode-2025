pub fn insert(intervals: Vec<Vec<i32>>, mut new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    let start = intervals
        .iter()
        .position(|interval| interval[1] >= new_interval[0])
        .unwrap_or(intervals.len());
    let end = intervals
        .iter()
        .position(|interval| interval[0] > new_interval[1])
        .unwrap_or(intervals.len());

    if start < end {
        new_interval[0] = new_interval[0].min(intervals[start][0]);
        new_interval[1] = new_interval[1].max(intervals[end - 1][1]);
    }

    [&intervals[..start], &[new_interval], &intervals[end..]].concat()
}
