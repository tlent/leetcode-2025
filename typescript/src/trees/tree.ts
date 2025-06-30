export type Tree = TreeNode | null;

export class TreeNode {
  value: number;
  left: Tree;
  right: Tree;

  constructor(value = 0, left: Tree = null, right: Tree = null) {
    this.value = value;
    this.left = left;
    this.right = right;
  }

  static from_array(values: Iterable<number>): Tree {
    const nodes = Array.from(values).map((value) => new TreeNode(value));
    if (nodes.length === 0) {
      return null;
    }
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
    return nodes[0]!;
  }

  *nodes(): Generator<TreeNode> {
    const queue: TreeNode[] = [this];
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
}
