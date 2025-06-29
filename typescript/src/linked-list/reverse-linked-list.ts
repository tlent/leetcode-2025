import { List, ListNode } from "./list";

function reverseList(head: List): List {
  let prev = null;
  let cursor = head;
  while (cursor) {
    const next = cursor.next;
    cursor.next = prev;
    prev = cursor;
    cursor = next;
  }
  return prev;
}

function reverseListRecursive(head: List): List {
  if (!head?.next) {
    return head;
  }
  const newHead = reverseListRecursive(head.next);
  head.next.next = head;
  head.next = null;
  return newHead;
}

if (import.meta.vitest) {
  const { test, expect } = await import("vitest");

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
}
