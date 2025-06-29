import { test, expect } from "vitest";
import { hasCycle } from "./linked-list-cycle";
import { createCycleList } from "../test-utils";

test("hasCycle example 1", () => {
  const list = createCycleList([3, 2, 0, -4], 1);
  expect(hasCycle(list)).toBe(true);
});

test("hasCycle example 2", () => {
  const list = createCycleList([1, 2], 0);
  expect(hasCycle(list)).toBe(true);
});

test("hasCycle example 3", () => {
  const list = createCycleList([1], -1);
  expect(hasCycle(list)).toBe(false);
});
