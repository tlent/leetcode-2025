import { expect, test } from 'bun:test';

import { middleNode } from './n0876_middle_of_the_linked_list';
import { List } from './utils/linked_list';

test('middleNode', () => {
  let list = new List([1, 2, 3, 4, 5]);
  expect(middleNode(list.head)?.value).toEqual(3);
  list = new List([1, 2, 3, 4, 5, 6]);
  expect(middleNode(list.head)?.value).toEqual(4);
});
