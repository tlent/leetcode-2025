import { Queue } from '@datastructures-js/queue';

export function updateMatrix(matrix: number[][]): number[][] {
  const m = matrix.length;
  const n = matrix[0]!.length;
  const result: number[][] = Array(m)
    .fill(null)
    .map(() => Array(n).fill(-1));
  const queue = new Queue<[number, number]>();
  for (let i = 0; i < m; i++) {
    for (let j = 0; j < n; j++) {
      if (matrix[i]![j] === 0) {
        result[i]![j] = 0;
        queue.push([i, j]);
      }
    }
  }
  while (!queue.isEmpty()) {
    const [i, j] = queue.pop();
    if (i > 0 && result[i - 1]![j] === -1) {
      result[i - 1]![j] = result[i]![j]! + 1;
      queue.push([i - 1, j]);
    }
    if (j > 0 && result[i]![j - 1] === -1) {
      result[i]![j - 1] = result[i]![j]! + 1;
      queue.push([i, j - 1]);
    }
    if (i + 1 < m && result[i + 1]![j] === -1) {
      result[i + 1]![j] = result[i]![j]! + 1;
      queue.push([i + 1, j]);
    }
    if (j + 1 < n && result[i]![j + 1] === -1) {
      result[i]![j + 1] = result[i]![j]! + 1;
      queue.push([i, j + 1]);
    }
  }
  return result;
}
