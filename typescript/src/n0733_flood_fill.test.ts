import { expect, test } from 'bun:test';
import { floodFill } from './n0733_flood_fill';

test('floodFill', () => {
  expect(
    floodFill(
      [
        [1, 1, 1],
        [1, 1, 0],
        [1, 0, 1],
      ],
      1,
      1,
      2
    )
  ).toEqual([
    [2, 2, 2],
    [2, 2, 0],
    [2, 0, 1],
  ]);

  expect(
    floodFill(
      [
        [0, 0, 0],
        [0, 0, 0],
      ],
      0,
      0,
      0
    )
  ).toEqual([
    [0, 0, 0],
    [0, 0, 0],
  ]);
});
