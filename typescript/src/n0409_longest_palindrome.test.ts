import { expect, test } from 'bun:test';

import { longestPalindrome } from './n0409_longest_palindrome';

test('longestPalindrome', () => {
  expect(longestPalindrome('abccccdd')).toEqual(7);
  expect(longestPalindrome('a')).toEqual(1);
});
