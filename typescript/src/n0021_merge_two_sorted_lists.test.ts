import { expect } from "jsr:@std/expect";
import { merge } from "./n0021_merge_two_sorted_lists.ts";
import { List } from "./utils/linked_list.ts";

Deno.test("merge", () => {
  const a = new List([1, 2, 4]);
  const b = new List([1, 3, 4]);
  a.head = merge(a.head, b.head);
  expect(a.toArray()).toEqual([1, 1, 2, 3, 4, 4]);
});
