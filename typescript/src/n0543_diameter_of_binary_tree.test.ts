import { expect, test } from 'bun:test';

import { diameterOfBinaryTree } from './n0543_diameter_of_binary_tree';
import { Tree } from './utils/tree';

test('diameterOfBinaryTree', () => {
  let tree = new Tree([1, 2, 3, 4, 5]);
  expect(diameterOfBinaryTree(tree.root)).toEqual(3);
  tree = new Tree([1, 2]);
  expect(diameterOfBinaryTree(tree.root)).toEqual(1);
});
