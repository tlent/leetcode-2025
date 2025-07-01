import type { TreeNode } from './utils/tree';

export function invertTree(root: TreeNode | null): TreeNode | null {
  if (!root) {
    return null;
  }
  const stack = [root];
  while (stack.length > 0) {
    const node = stack.pop()!;
    [node.left, node.right] = [node.right, node.left];
    if (node.left) {
      stack.push(node.left);
    }
    if (node.right) {
      stack.push(node.right);
    }
  }
  return root;
}

export function invertTreeRecursive(root: TreeNode | null): TreeNode | null {
  if (!root) {
    return null;
  }
  [root.left, root.right] = [
    invertTreeRecursive(root.right),
    invertTreeRecursive(root.left),
  ];
  return root;
}
