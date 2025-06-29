import { test, expect } from "vitest";
import { hasCycle } from "./linked-list-cycle";
import { List, ListNode } from "./list";

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

function createCycleList(values: number[], cyclePos: number): List {
  const list = ListNode.from_values(values);
  if (cyclePos < 0 || !list) {
    return list;
  }

  const nodes = Array.from(list.nodes());
  if (cyclePos < nodes.length) {
    nodes[nodes.length - 1]!.next = nodes[cyclePos]!;
  }

  return list;
}
