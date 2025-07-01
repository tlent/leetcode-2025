import { expect, test } from 'bun:test';
import { valid_parentheses } from './n0020_valid_parentheses';

test('isValid', () => {
  expect(valid_parentheses('()')).toBe(true);
  expect(valid_parentheses('()[]{}')).toBe(true);
  expect(valid_parentheses('(]')).toBe(false);
  expect(valid_parentheses('([])')).toBe(true);
});
