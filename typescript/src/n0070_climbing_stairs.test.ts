import { expect, test } from 'bun:test';
import { climbStairs } from './n0070_climbing_stairs';

test('climbStairs', () => {
  expect(climbStairs(2)).toEqual(2);
  expect(climbStairs(3)).toEqual(3);
});
