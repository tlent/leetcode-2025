import type { ListNode } from "./utils/linked_list.ts";

export function reverseList(head: ListNode | null): ListNode | null {
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

export function reverseListRecursive(head: ListNode | null): ListNode | null {
  if (!head?.next) {
    return head;
  }
  const newHead = reverseListRecursive(head.next);
  head.next.next = head;
  head.next = null;
  return newHead;
}
