import { List } from "./list";

export function hasCycle(head: List): boolean {
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

