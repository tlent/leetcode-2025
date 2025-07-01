import { expect, test } from 'bun:test';
import { binarySearch } from './n0704_binary_search';

test('binarySearch', () => {
  expect(binarySearch([-1, 0, 3, 5, 9, 12], 9)).toEqual(4);
  expect(binarySearch([-1, 0, 3, 5, 9, 12], 2)).toEqual(-1);
});
