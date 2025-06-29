import { List, ListNode } from "./linked-list/list";

/**
 * Test utilities for LeetCode problems
 */

/**
 * Creates a linked list with a cycle at the specified position
 * @param values - Array of values to create the list from
 * @param cyclePos - Index where the cycle should connect back to (0-based), -1 for no cycle
 * @returns The head of the linked list with cycle created
 */
export function createCycleList(values: number[], cyclePos: number): List {
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

/**
 * Converts a linked list to an array of values for easy testing
 * @param head - Head of the linked list
 * @param maxLength - Maximum length to prevent infinite loops in cyclic lists
 * @returns Array of values
 */
export function listToArray(head: List, maxLength = 100): number[] {
  const result: number[] = [];
  let current = head;
  let count = 0;
  
  while (current && count < maxLength) {
    result.push(current.val);
    current = current.next;
    count++;
  }
  
  return result;
}