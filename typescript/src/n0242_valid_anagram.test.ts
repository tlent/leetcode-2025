import { expect, test } from 'bun:test';
import { isAnagram } from './n0242_valid_anagram';

test('isAnagram', () => {
  expect(isAnagram('anagram', 'nagaram')).toBe(true);
  expect(isAnagram('rat', 'car')).toBe(false);
  expect(isAnagram('listen', 'silent')).toBe(true);
  expect(isAnagram('a', 'ab')).toBe(false);
});
