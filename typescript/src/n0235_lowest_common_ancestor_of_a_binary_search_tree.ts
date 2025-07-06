import type { TreeNode } from './utils/tree';

export function lowestCommonAncestor(
  root: TreeNode | null,
  p: TreeNode | null,
  q: TreeNode | null
): TreeNode | null {
  const min = Math.min(p!.value, q!.value);
  const max = Math.max(p!.value, q!.value);
  let node = root;
  while (node) {
    if (max < node.value) {
      node = node.left;
    } else if (min > node.value) {
      node = node.right;
    } else {
      break;
    }
  }
  return node;
}
