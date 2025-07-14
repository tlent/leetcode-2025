import { expect, test } from 'bun:test';

import { addBinary } from './n0067_add_binary';

test('addBinary', () => {
  expect(addBinary('11', '1')).toEqual('100');
  expect(addBinary('1010', '1011')).toEqual('10101');
});
