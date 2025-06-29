import { List, ListNode } from "./list";

function hasCycle(head: List): boolean {
  let slow = head;
  let fast = head;
  while (slow && fast?.next) {
    slow = slow.next;
    fast = fast.next.next;
    if (slow === fast) {
      return true;
    }
  }
  return false;
}

if (import.meta.vitest) {
  const { expect, test } = await import("vitest");

  test("hasCycle example 1", () => {
    const list = ListNode.from_values([3, 2, 0, -4])!;
    const nodes = Array.from(list.nodes());
    nodes[nodes.length - 1]!.next = nodes[1]!;
    expect(hasCycle(list)).toBe(true);
  });

  test("hasCycle example 2", () => {
    const list = ListNode.from_values([1, 2])!;
    const nodes = Array.from(list.nodes());
    nodes[nodes.length - 1]!.next = nodes[0]!;
    expect(hasCycle(list)).toBe(true);
  });

  test("hasCycle example 3", () => {
    const list = ListNode.from_values([1])!;
    expect(hasCycle(list)).toBe(false);
  });
}
