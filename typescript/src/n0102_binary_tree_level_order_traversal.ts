import { Queue } from '@datastructures-js/queue';

import type { TreeNode } from './utils/tree';

export function levelOrder(root: TreeNode | null): number[][] {
  if (!root) {
    return [];
  }
  const result = [];
  const queue = new Queue([root]);
  while (!queue.isEmpty()) {
    const level = [];
    const len = queue.size();
    for (let i = 0; i < len; i++) {
      const node = queue.pop();
      level.push(node.value);
      if (node.left) {
        queue.push(node.left);
      }
      if (node.right) {
        queue.push(node.right);
      }
    }
    result.push(level);
  }
  return result;
}
