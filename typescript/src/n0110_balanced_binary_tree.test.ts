import { expect, test } from 'bun:test';
import { isBalanced } from './n0110_balanced_binary_tree';
import { Tree } from './utils/tree';

test('isBalanced example one', () => {
  const tree = new Tree([3, 9, 20, null, null, 15, 7]);
  expect(isBalanced(tree.root)).toBe(true);
});

test('isBalanced example two', () => {
  const tree = new Tree([1, 2, 2, 3, 3, null, null, 4, 4]);
  expect(isBalanced(tree.root)).toBe(false);
});

test('isBalanced example three', () => {
  const tree = new Tree([]);
  expect(isBalanced(tree.root)).toBe(true);
});

test('isBalanced example four', () => {
  const tree = new Tree([1, 2, 2, 3, null, null, 3, 4, null, null, 4]);
  expect(isBalanced(tree.root)).toBe(false);
});
