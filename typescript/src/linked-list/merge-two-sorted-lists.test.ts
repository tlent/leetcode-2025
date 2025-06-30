import { expect, test } from 'bun:test';
import { List } from './linked-list';
import { merge } from './merge-two-sorted-lists';

test('merge', () => {
  const a = new List([1, 2, 4]);
  const b = new List([1, 3, 4]);
  a.head = merge(a.head, b.head);
  expect(a.toArray()).toEqual([1, 1, 2, 3, 4, 4]);
});
