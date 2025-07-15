import type { ListNode } from './utils/linked_list';

export function middleNode(head: ListNode | null): ListNode | null {
  let fast = head;
  let slow = head;

  while (fast?.next) {
    slow = slow!.next;
    fast = fast.next.next;
  }

  return slow;
}
