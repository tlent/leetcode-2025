import { expect, test } from 'bun:test';
import { isAnagram } from './valid-anagram';

test('isAnagram', () => {
  expect(isAnagram('anagram', 'nagaram')).toBe(true);
  expect(isAnagram('rat', 'car')).toBe(false);
  expect(isAnagram('listen', 'silent')).toBe(true);
  expect(isAnagram('a', 'ab')).toBe(false);
});
