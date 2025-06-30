export class ListNode {
  value: number;
  next: ListNode | null;

  constructor(value = 0, next: ListNode | null = null) {
    this.value = value;
    this.next = next;
  }
}

export class List {
  public head: ListNode | null;

  constructor(values: number[]) {
    let head: ListNode | null = null;
    let cursor: ListNode | null = null;
    for (const val of values) {
      const node = new ListNode(val);
      if (!cursor) {
        head = node;
      } else {
        cursor.next = node;
      }
      cursor = node;
    }
    this.head = head;
  }

  *nodes(): Generator<ListNode> {
    let cursor = this.head;
    while (cursor) {
      yield cursor;
      cursor = cursor.next;
    }
  }

  *values(): Generator<number> {
    for (const node of this.nodes()) {
      yield node.value;
    }
  }

  toArray(): number[] {
    return Array.from(this.values());
  }
}
