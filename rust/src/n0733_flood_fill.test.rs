use crate::n0733_flood_fill::flood_fill;

#[test]
fn test_flood_fill() {
    assert_eq!(
        flood_fill(vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]], 1, 1, 2),
        [vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]]
    );
    assert_eq!(
        flood_fill(vec![vec![0, 0, 0], vec![0, 0, 0]], 0, 0, 0),
        [vec![0, 0, 0], vec![0, 0, 0]]
    );
}
