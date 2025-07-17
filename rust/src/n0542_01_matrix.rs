use std::collections::VecDeque;

pub fn update_matrix(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let (m, n) = (matrix.len(), matrix[0].len());
    let mut result: Vec<Vec<i32>> = vec![vec![-1; n]; m];
    let mut queue = VecDeque::new();
    for (i, row) in matrix.iter().enumerate() {
        for (j, &value) in row.iter().enumerate() {
            if value == 0 {
                result[i][j] = 0;
                queue.push_back((i, j));
            }
        }
    }
    while let Some((i, j)) = queue.pop_front() {
        if i > 0 && result[i - 1][j] == -1 {
            result[i - 1][j] = result[i][j] + 1;
            queue.push_back((i - 1, j));
        }
        if j > 0 && result[i][j - 1] == -1 {
            result[i][j - 1] = result[i][j] + 1;
            queue.push_back((i, j - 1));
        }
        if i + 1 < m && result[i + 1][j] == -1 {
            result[i + 1][j] = result[i][j] + 1;
            queue.push_back((i + 1, j));
        }
        if j + 1 < n && result[i][j + 1] == -1 {
            result[i][j + 1] = result[i][j] + 1;
            queue.push_back((i, j + 1));
        }
    }
    result
}
