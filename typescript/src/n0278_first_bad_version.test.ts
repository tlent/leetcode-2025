import { expect, test } from 'bun:test';

import { firstBadVersion } from './n0278_first_bad_version';

test('firstBadVersion example one', () => {
  expect(firstBadVersion(5, (v) => v >= 4)).toEqual(4);
});
test('firstBadVersion example two', () => {
  expect(firstBadVersion(1, (v) => v >= 1)).toEqual(1);
});
