import { expect } from "jsr:@std/expect";
import { isBalanced } from "./n0110_balanced_binary_tree.ts";
import { Tree } from "./utils/tree.ts";

Deno.test("isBalanced example one", () => {
  const tree = new Tree([3, 9, 20, null, null, 15, 7]);
  expect(isBalanced(tree.root)).toBe(true);
});

Deno.test("isBalanced example two", () => {
  const tree = new Tree([1, 2, 2, 3, 3, null, null, 4, 4]);
  expect(isBalanced(tree.root)).toBe(false);
});

Deno.test("isBalanced example three", () => {
  const tree = new Tree([]);
  expect(isBalanced(tree.root)).toBe(true);
});

Deno.test("isBalanced example four", () => {
  const tree = new Tree([1, 2, 2, 3, null, null, 3, 4, null, null, 4]);
  expect(isBalanced(tree.root)).toBe(false);
});
