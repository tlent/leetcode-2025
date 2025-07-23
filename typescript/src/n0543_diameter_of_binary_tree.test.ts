import { expect } from "jsr:@std/expect";

import { diameterOfBinaryTree } from "./n0543_diameter_of_binary_tree.ts";
import { Tree } from "./utils/tree.ts";

Deno.test("diameterOfBinaryTree", () => {
  let tree = new Tree([1, 2, 3, 4, 5]);
  expect(diameterOfBinaryTree(tree.root)).toEqual(3);
  tree = new Tree([1, 2]);
  expect(diameterOfBinaryTree(tree.root)).toEqual(1);
});
