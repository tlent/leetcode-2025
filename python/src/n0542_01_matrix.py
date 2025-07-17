from collections import deque


def update_matrix(matrix: list[list[int]]) -> list[list[int]]:
    m = len(matrix)
    n = len(matrix[0])
    result = [[-1 for _ in range(n)] for _ in range(m)]
    queue: deque[tuple[int, int]] = deque()
    for i in range(m):
        for j in range(n):
            if matrix[i][j] == 0:
                result[i][j] = 0
                queue.append((i, j))
    while queue:
        (i, j) = queue.popleft()
        if i > 0 and result[i - 1][j] == -1:
            result[i - 1][j] = result[i][j] + 1
            queue.append((i - 1, j))
        if j > 0 and result[i][j - 1] == -1:
            result[i][j - 1] = result[i][j] + 1
            queue.append((i, j - 1))
        if i + 1 < m and result[i + 1][j] == -1:
            result[i + 1][j] = result[i][j] + 1
            queue.append((i + 1, j))
        if j + 1 < n and result[i][j + 1] == -1:
            result[i][j + 1] = result[i][j] + 1
            queue.append((i, j + 1))
    return result
