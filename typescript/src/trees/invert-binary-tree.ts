import { Tree } from "./tree";

export function invertTree(root: Tree): Tree {
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

export function invertTreeRecursive(root: Tree): Tree {
  if (!root) {
    return null;
  }
  [root.left, root.right] = [
    invertTreeRecursive(root.right),
    invertTreeRecursive(root.left),
  ];
  return root;
}
