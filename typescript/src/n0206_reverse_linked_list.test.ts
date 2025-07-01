import { expect, test } from 'bun:test';
import { reverseList, reverseListRecursive } from './n0206_reverse_linked_list';
import { List } from './utils/linked_list';

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
