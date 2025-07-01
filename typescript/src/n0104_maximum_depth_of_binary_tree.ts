import type { TreeNode } from './utils/tree';

export function maxDepth(root: TreeNode | null): number {
  if (!root) {
    return 0;
  }
  return 1 + Math.max(maxDepth(root.left), maxDepth(root.right));
}

export function maxDepthIterative(root: TreeNode | null): number {
  if (!root) {
    return 0;
  }
  const stack = [{ node: root, depth: 1 }];
  let maxDepth = 1;
  while (stack.length > 0) {
    const { node, depth } = stack.pop()!;
    maxDepth = Math.max(maxDepth, depth);
    if (node.left) {
      stack.push({ node: node.left, depth: depth + 1 });
    }
    if (node.right) {
      stack.push({ node: node.right, depth: depth + 1 });
    }
  }
  return maxDepth;
}
