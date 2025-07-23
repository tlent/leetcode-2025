import { expect } from "jsr:@std/expect";
import {
  maxDepth,
  maxDepthIterative,
} from "./n0104_maximum_depth_of_binary_tree.ts";
import { Tree } from "./utils/tree.ts";

Deno.test("maxDepth example one", () => {
  const tree = new Tree([3, 9, 20, null, null, 15, 7]);
  expect(maxDepth(tree.root)).toEqual(3);
});

Deno.test("maxDepth example two", () => {
  const tree = new Tree([1, null, 2]);
  expect(maxDepth(tree.root)).toEqual(2);
});

Deno.test("maxDepthIterative example one", () => {
  const tree = new Tree([3, 9, 20, null, null, 15, 7]);
  expect(maxDepthIterative(tree.root)).toEqual(3);
});

Deno.test("maxDepthIterative example two", () => {
  const tree = new Tree([1, null, 2]);
  expect(maxDepthIterative(tree.root)).toEqual(2);
});
