import { expect, test } from 'bun:test';
import { List } from './linked-list';
import { reverseList, reverseListRecursive } from './reverse-linked-list';

test('reverseList', () => {
  const list = new List([1, 2, 3, 4, 5]);
  list.head = reverseList(list.head);
  expect(list.toArray()).toEqual([5, 4, 3, 2, 1]);
});

test('reverseListRecursive', () => {
  const list = new List([1, 2, 3, 4, 5]);
  list.head = reverseListRecursive(list.head);
  expect(list.toArray()).toEqual([5, 4, 3, 2, 1]);
});
