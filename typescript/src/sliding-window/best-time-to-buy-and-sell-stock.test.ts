import { expect, test } from 'bun:test';
import { maxProfit } from './best-time-to-buy-and-sell-stock';

test('maxProfit', () => {
  expect(maxProfit([7, 1, 5, 3, 6, 4])).toEqual(5);
  expect(maxProfit([7, 6, 4, 3, 1])).toEqual(0);
});
