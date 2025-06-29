import { List, ListNode } from "./list";

export function merge(a: List, b: List): List {
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

