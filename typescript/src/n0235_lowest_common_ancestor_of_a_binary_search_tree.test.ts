import { expect, test } from 'bun:test';

import { lowestCommonAncestor } from './n0235_lowest_common_ancestor_of_a_binary_search_tree';
import { Tree } from './utils/tree';

test('lowestCommonAncestor example one', () => {
  const tree = new Tree([6, 2, 8, 0, 4, 7, 9, null, null, 3, 5]);

  const nodes = Array.from(tree.nodes());
  const p = nodes.find((node) => node.value === 2)!;
  const q = nodes.find((node) => node.value === 8)!;
  const expected = nodes.find((node) => node.value === 6)!;

  expect(lowestCommonAncestor(tree.root, p, q)).toEqual(expected);
});

test('lowestCommonAncestor example two', () => {
  const tree = new Tree([6, 2, 8, 0, 4, 7, 9, null, null, 3, 5]);

  const nodes = Array.from(tree.nodes());
  const p = nodes.find((node) => node.value === 2)!;
  const q = nodes.find((node) => node.value === 4)!;
  const expected = nodes.find((node) => node.value === 2)!;

  expect(lowestCommonAncestor(tree.root, p, q)).toEqual(expected);
});

test('lowestCommonAncestor example three', () => {
  const tree = new Tree([2, 1]);

  const nodes = Array.from(tree.nodes());
  const p = nodes.find((node) => node.value === 2)!;
  const q = nodes.find((node) => node.value === 1)!;
  const expected = nodes.find((node) => node.value === 2)!;

  expect(lowestCommonAncestor(tree.root, p, q)).toEqual(expected);
});
