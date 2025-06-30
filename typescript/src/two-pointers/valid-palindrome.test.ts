import { expect, test } from 'bun:test';
import { isPalindrome } from './valid-palindrome';

test('isPalindrome', () => {
  expect(isPalindrome('A man, a plan, a canal: Panama')).toBe(true);
  expect(isPalindrome('race a car')).toBe(false);
  expect(isPalindrome(' ')).toBe(true);
});
