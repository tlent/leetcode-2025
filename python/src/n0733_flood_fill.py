def flood_fill(
    image: list[list[int]], start_row: int, start_col: int, fill_color: int
) -> list[list[int]]:
    start_color = image[start_row][start_col]
    if start_color == fill_color:
        return image
    image[start_row][start_col] = fill_color
    stack = [(start_row, start_col)]
    while stack:
        (row, col) = stack.pop()
        adjacent = [(row - 1, col), (row, col - 1), (row + 1, col), (row, col + 1)]
        for r, c in adjacent:
            if (
                (0 <= r < len(image))
                and (0 <= c < len(image[0]))
                and image[r][c] == start_color
            ):
                image[r][c] = fill_color
                stack.append((r, c))
    return image
