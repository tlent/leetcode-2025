import type { TreeNode } from "./utils/tree.ts";

export function diameterOfBinaryTree(root: TreeNode | null): number {
  let max_diameter = 0;

  function depth(root: TreeNode | null): number {
    if (!root) {
      return 0;
    }

    const left = depth(root.left);
    const right = depth(root.right);

    max_diameter = Math.max(max_diameter, left + right);

    return 1 + Math.max(left, right);
  }
  depth(root);

  return max_diameter;
}
