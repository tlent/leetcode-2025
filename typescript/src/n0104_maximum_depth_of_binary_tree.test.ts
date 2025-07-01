import { expect, test } from 'bun:test';
import {
  maxDepth,
  maxDepthIterative,
} from './n0104_maximum_depth_of_binary_tree';
import { Tree } from './utils/tree';

test('maxDepth example one', () => {
  const tree = new Tree([3, 9, 20, null, null, 15, 7]);
  expect(maxDepth(tree.root)).toEqual(3);
});

test('maxDepth example two', () => {
  const tree = new Tree([1, null, 2]);
  expect(maxDepth(tree.root)).toEqual(2);
});

test('maxDepthIterative example one', () => {
  const tree = new Tree([3, 9, 20, null, null, 15, 7]);
  expect(maxDepthIterative(tree.root)).toEqual(3);
});

test('maxDepthIterative example two', () => {
  const tree = new Tree([1, null, 2]);
  expect(maxDepthIterative(tree.root)).toEqual(2);
});
