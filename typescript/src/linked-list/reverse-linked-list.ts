class ListNode {
  val: number;
  next: ListNode | null;
  
  constructor(val = 0, next: ListNode | null = null) {
    this.val = val;
    this.next = next;
  }
}

function reverseList(head: ListNode | null): ListNode | null {
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

function reverseListRecursive(head: ListNode | null): ListNode | null {
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
    const list = new ListNode(1, new ListNode(2, new ListNode(3, new ListNode(4, new ListNode(5)))));

    let expected: ListNode | null = new ListNode(5, new ListNode(4, new ListNode(3, new ListNode(2, new ListNode(1)))));
    let actual = reverseList(list);

    while (expected && actual) {
      expect(expected.val).toEqual(actual.val);
      expected = expected.next;
      actual = actual.next;
    }

    expect(expected).toBeNull();
    expect(actual).toBeNull();
  });

  test("reverseListRecursive", () => {
    const list = new ListNode(1, new ListNode(2, new ListNode(3, new ListNode(4, new ListNode(5)))));

    let expected: ListNode | null = new ListNode(5, new ListNode(4, new ListNode(3, new ListNode(2, new ListNode(1)))));
    let actual = reverseListRecursive(list);

    while (expected && actual) {
      expect(expected.val).toEqual(actual.val);
      expected = expected.next;
      actual = actual.next;
    }

    expect(expected).toBeNull();
    expect(actual).toBeNull();
  });
}