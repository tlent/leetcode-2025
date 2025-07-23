import { expect } from "jsr:@std/expect";
import { invertTree, invertTreeRecursive } from "./n0226_invert_binary_tree.ts";
import { Tree } from "./utils/tree.ts";

Deno.test("invertTree example one", () => {
  const tree = new Tree([4, 2, 7, 1, 3, 6, 9]);
  tree.root = invertTree(tree.root);
  expect(tree.toArray()).toEqual([4, 7, 2, 9, 6, 3, 1]);
});

Deno.test("invertTree example two", () => {
  const tree = new Tree([2, 1, 3]);
  tree.root = invertTree(tree.root);
  expect(tree.toArray()).toEqual([2, 3, 1]);
});

Deno.test("invertTree example three", () => {
  const tree = new Tree([]);
  tree.root = invertTree(tree.root);
  expect(tree.root).toBeNull();
});

Deno.test("invertTreeRecursive example one", () => {
  const tree = new Tree([4, 2, 7, 1, 3, 6, 9]);
  tree.root = invertTreeRecursive(tree.root);
  expect(tree.toArray()).toEqual([4, 7, 2, 9, 6, 3, 1]);
});

Deno.test("invertTreeRecursive example two", () => {
  const tree = new Tree([2, 1, 3]);
  tree.root = invertTreeRecursive(tree.root);
  expect(tree.toArray()).toEqual([2, 3, 1]);
});

Deno.test("invertTreeRecursive example three", () => {
  const tree = new Tree([]);
  tree.root = invertTreeRecursive(tree.root);
  expect(tree.root).toBeNull();
});
