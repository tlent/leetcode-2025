import { List, ListNode } from "./list";

function merge(a: List, b: List): List {
  const head = new ListNode();
  let cursor = head;
  while (a && b) {
    if (a.val <= b.val) {
      cursor.next = a;
      a = a.next;
    } else {
      cursor.next = b;
      b = b.next;
    }
    cursor = cursor.next;
  }
  cursor.next = a ?? b;
  return head.next;
}

if (import.meta.vitest) {
  const { test, expect } = await import("vitest");

  test("merge", () => {
    const a = ListNode.from_values([1, 2, 4]);
    const b = ListNode.from_values([1, 3, 4]);
    const result = merge(a, b);
    expect(Array.from(result!.values())).toEqual([1, 1, 2, 3, 4, 4]);
  });
}
