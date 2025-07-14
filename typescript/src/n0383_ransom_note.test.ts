import { expect, test } from 'bun:test';
import { canConstruct } from './n0383_ransom_note';

test('canConstruct', () => {
  expect(canConstruct('a', 'b')).toBeFalse();
  expect(canConstruct('aa', 'ab')).toBeFalse();
  expect(canConstruct('aa', 'aab')).toBeTrue();
});
