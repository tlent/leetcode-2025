import type { ListNode } from './linked-list';

export function hasCycle(head: ListNode | null): boolean {
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
