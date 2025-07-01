export function floodFill(
  image: number[][],
  startRow: number,
  startCol: number,
  fillColor: number
): number[][] {
  const startColor = image[startRow]![startCol];
  if (startColor === fillColor) {
    return image;
  }
  image[startRow]![startCol] = fillColor;
  const stack: [number, number][] = [[startRow, startCol]];
  while (stack.length > 0) {
    const [row, col] = stack.pop()!;
    const adjacent: [number, number][] = [
      [row - 1, col],
      [row, col - 1],
      [row + 1, col],
      [row, col + 1],
    ];
    for (const [r, c] of adjacent) {
      if (
        0 <= r &&
        r < image.length &&
        0 <= c &&
        c < image[0]!.length &&
        image[r]![c] === startColor
      ) {
        image[r]![c] = fillColor;
        stack.push([r, c]);
      }
    }
  }
  return image;
}
