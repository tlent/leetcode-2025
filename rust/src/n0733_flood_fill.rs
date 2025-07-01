pub fn flood_fill(
    mut image: Vec<Vec<i32>>,
    start_row: usize,
    start_col: usize,
    fill_color: i32,
) -> Vec<Vec<i32>> {
    let start_color = image[start_row][start_col];
    if start_color == fill_color {
        return image;
    }
    image[start_row][start_col] = fill_color;
    let mut stack = vec![(start_row, start_col)];
    while let Some((row, col)) = stack.pop() {
        let adjacent = [
            (row.wrapping_sub(1), col),
            (row, col.wrapping_sub(1)),
            (row + 1, col),
            (row, col + 1),
        ];
        for (r, c) in adjacent {
            if r < image.len() && c < image[0].len() && image[r][c] == start_color {
                image[r][c] = fill_color;
                stack.push((r, c));
            }
        }
    }
    image
}
