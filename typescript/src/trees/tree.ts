export class TreeNode {
  value: number;
  left: TreeNode | null;
  right: TreeNode | null;

  constructor(
    value = 0,
    left: TreeNode | null = null,
    right: TreeNode | null = null,
  ) {
    this.value = value;
    this.left = left;
    this.right = right;
  }
}

export class Tree {
  public root: TreeNode | null;

  constructor(values: Iterable<number>) {
    const nodes = Array.from(values).map((value) => new TreeNode(value));
    if (nodes.length === 0) {
      this.root = null;
    } else {
      for (const [i, node] of nodes.entries()) {
        const left_index = i * 2 + 1;
        if (left_index < nodes.length) {
          node.left = nodes[left_index]!;
        }
        const right_index = left_index + 1;
        if (right_index < nodes.length) {
          node.right = nodes[right_index]!;
        }
      }
      this.root = nodes[0]!;
    }
  }

  *nodes(): Generator<TreeNode> {
    if (!this.root) return;
    const queue: TreeNode[] = [this.root];
    while (queue.length > 0) {
      const node = queue.shift()!;
      if (node.left) {
        queue.push(node.left);
      }
      if (node.right) {
        queue.push(node.right);
      }
      yield node;
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
