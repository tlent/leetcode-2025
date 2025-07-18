use std::collections::BinaryHeap;

fn distance(point: &[i32]) -> i32 {
    point[0].pow(2) + point[1].pow(2)
}

// O(n log n)
pub fn k_closest(mut points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    points.sort_unstable_by_key(|point| distance(point));
    points.truncate(k as usize);
    points
}

// O(n log k)
pub fn k_closest_heap(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let mut heap = BinaryHeap::with_capacity(k as usize + 1);
    for point in points {
        heap.push((distance(&point), point));
        if heap.len() > k as usize {
            heap.pop();
        }
    }
    heap.into_iter().map(|(_, point)| point).collect()
}
