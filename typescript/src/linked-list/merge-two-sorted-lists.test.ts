import { test, expect } from "vitest";
import { merge } from "./merge-two-sorted-lists";
import { ListNode } from "./list";

test("merge", () => {
  const a = ListNode.from_values([1, 2, 4]);
  const b = ListNode.from_values([1, 3, 4]);
  const result = merge(a, b);
  expect(Array.from(result!.values())).toEqual([1, 1, 2, 3, 4, 4]);
});