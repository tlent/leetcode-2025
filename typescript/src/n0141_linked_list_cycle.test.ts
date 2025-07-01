import { expect, test } from 'bun:test';
import { hasCycle } from './n0141_linked_list_cycle';
import { List } from './utils/linked_list';

test('hasCycle example 1', () => {
  const list = new List([3, 2, 0, -4]);

  const nodes = Array.from(list.nodes());
  nodes[nodes.length - 1]!.next = nodes[1]!;

  expect(hasCycle(list.head)).toBe(true);
});

test('hasCycle example 2', () => {
  const list = new List([1, 2]);

  const nodes = Array.from(list.nodes());
  nodes[nodes.length - 1]!.next = nodes[0]!;

  expect(hasCycle(list.head)).toBe(true);
});

test('hasCycle example 3', () => {
  const list = new List([1]);
  expect(hasCycle(list.head)).toBe(false);
});
