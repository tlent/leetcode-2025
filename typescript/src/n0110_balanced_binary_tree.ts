import type { TreeNode } from './utils/tree';

export function isBalanced(root: TreeNode | null): boolean {
  return checkBalance(root) !== null;
}

function checkBalance(node: TreeNode | null): number | null {
  if (!node) {
    return 0;
  }
  const left = checkBalance(node.left);
  if (left === null) {
    return null;
  }
  const right = checkBalance(node.right);
  if (right === null || Math.abs(left - right) > 1) {
    return null;
  }
  return 1 + Math.max(left, right);
}
