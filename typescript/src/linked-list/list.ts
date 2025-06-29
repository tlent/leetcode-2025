export type List = ListNode | null;

export class ListNode {
  val: number;
  next: List;

  constructor(val = 0, next: List = null) {
    this.val = val;
    this.next = next;
  }

  static from_values(iterable: Iterable<number>): List {
    let head: List = null;
    let cursor: List = null;
    for (const val of iterable) {
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
      yield node.val;
    }
  }
}
