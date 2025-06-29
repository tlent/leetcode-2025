export type List = ListNode | null;

export class ListNode {
  val: number;
  next: List;

  constructor(val = 0, next: List = null) {
    this.val = val;
    this.next = next;
  }

  static from(iterable: Iterable<number>): List {
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

  *[Symbol.iterator](): Generator<number> {
    let cursor: List = this;
    while (cursor) {
      yield cursor.val;
      cursor = cursor.next;
    }
  }
}
