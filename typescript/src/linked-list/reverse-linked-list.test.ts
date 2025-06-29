import { test, expect } from "vitest";
import { reverseList, reverseListRecursive } from "./reverse-linked-list";
import { ListNode } from "./list";

test("reverseList", () => {
  const list = ListNode.from_values([1, 2, 3, 4, 5]);
  const reversed = reverseList(list);
  expect(Array.from(reversed!.values())).toEqual([5, 4, 3, 2, 1]);
});

test("reverseListRecursive", () => {
  const list = ListNode.from_values([1, 2, 3, 4, 5]);
  const reversed = reverseListRecursive(list);
  expect(Array.from(reversed!.values())).toEqual([5, 4, 3, 2, 1]);
});
