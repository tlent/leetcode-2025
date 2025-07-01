import { ListNode } from './utils/linked_list';

export function merge(a: ListNode | null, b: ListNode | null): ListNode | null {
  const head = new ListNode();
  let cursor = head;
  while (a && b) {
    if (a.value <= b.value) {
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
