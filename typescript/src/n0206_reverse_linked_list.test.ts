import { expect } from "jsr:@std/expect";
import {
  reverseList,
  reverseListRecursive,
} from "./n0206_reverse_linked_list.ts";
import { List } from "./utils/linked_list.ts";

Deno.test("reverseList", () => {
  const list = new List([1, 2, 3, 4, 5]);
  list.head = reverseList(list.head);
  expect(list.toArray()).toEqual([5, 4, 3, 2, 1]);
});

Deno.test("reverseListRecursive", () => {
  const list = new List([1, 2, 3, 4, 5]);
  list.head = reverseListRecursive(list.head);
  expect(list.toArray()).toEqual([5, 4, 3, 2, 1]);
});
