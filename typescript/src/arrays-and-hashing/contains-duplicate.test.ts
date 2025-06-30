import { expect, test } from 'bun:test';
import { hasDuplicate, hasDuplicateEarlyReturn } from './contains-duplicate';

test('hasDuplicate', () => {
  expect(hasDuplicate([1, 2, 3, 1])).toBe(true);
  expect(hasDuplicate([1, 2, 3, 4])).toBe(false);
  expect(hasDuplicate([1, 1, 1, 3, 3, 4, 3, 2, 4, 2])).toBe(true);
});

test('hasDuplicateEarlyReturn', () => {
  expect(hasDuplicateEarlyReturn([1, 2, 3, 1])).toBe(true);
  expect(hasDuplicateEarlyReturn([1, 2, 3, 4])).toBe(false);
  expect(hasDuplicateEarlyReturn([1, 1, 1, 3, 3, 4, 3, 2, 4, 2])).toBe(true);
});
