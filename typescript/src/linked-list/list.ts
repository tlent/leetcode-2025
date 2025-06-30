export type List = ListNode | null;

export class ListNode {
  value: number;
  next: List;

  constructor(value = 0, next: List = null) {
    this.value = value;
    this.next = next;
  }

  static from_values(values: Iterable<number>): List {
    let head: List = null;
    let cursor: List = null;
    for (const val of values) {
      const node = new ListNode(val);
      if (!cursor) {
        head = node;
      } else {
        cursor.next = node;
      }
      cursor = node;
    }
    return head;
  }

  *nodes(): Generator<ListNode> {
    let cursor: List = this;
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
}
