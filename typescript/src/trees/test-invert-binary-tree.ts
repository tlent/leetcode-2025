import { expect, test } from "vitest";
import { TreeNode } from "./tree";
import { invertTree, invertTreeRecursive } from "./invert-binary-tree";

test("invertTree example one", () => {
  const root = TreeNode.from_values([4, 2, 7, 1, 3, 6, 9]);
  const inverted = invertTree(root);
  expect(Array.from(inverted!.values())).toEqual([4, 7, 2, 9, 6, 3, 1]);
});

test("invertTree example two", () => {
  const root = TreeNode.from_values([2, 1, 3]);
  const inverted = invertTree(root);
  expect(Array.from(inverted!.values())).toEqual([2, 3, 1]);
});

test("invertTree example three", () => {
  const root = TreeNode.from_values([]);
  const inverted = invertTree(root);
  expect(inverted).toBeNull();
});

test("invertTreeRecursive example one", () => {
  const root = TreeNode.from_values([4, 2, 7, 1, 3, 6, 9]);
  const inverted = invertTreeRecursive(root);
  expect(Array.from(inverted!.values())).toEqual([4, 7, 2, 9, 6, 3, 1]);
});

test("invertTreeRecursive example two", () => {
  const root = TreeNode.from_values([2, 1, 3]);
  const inverted = invertTreeRecursive(root);
  expect(Array.from(inverted!.values())).toEqual([2, 3, 1]);
});

test("invertTreeRecursive example three", () => {
  const root = TreeNode.from_values([]);
  const inverted = invertTreeRecursive(root);
  expect(inverted).toBeNull();
});
