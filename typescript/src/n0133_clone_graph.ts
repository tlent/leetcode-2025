import { Node } from "./utils/graph.ts";

export function cloneGraph(node: Node | null): Node | null {
  if (!node) {
    return null;
  }
  const newNodes: (Node | null)[] = Array(100).fill(null);
  const adjacencyLists: (number[] | null)[] = Array(100).fill(null);
  const visited: boolean[] = Array(100).fill(false);
  const stack = [node];
  visited[0] = true;
  while (stack.length > 0) {
    const node = stack.pop()!;
    const i = node.value - 1;
    newNodes[i] = new Node(node.value);
    adjacencyLists[i] = node.neighbors.map((node) => node.value);
    for (const neighbor of node.neighbors) {
      const j = neighbor.value - 1;
      if (!visited[j]) {
        visited[j] = true;
        stack.push(neighbor);
      }
    }
  }
  for (let i = 0; i < 100; i++) {
    if (newNodes[i]) {
      newNodes[i]!.neighbors = adjacencyLists[i]!.map((j) => newNodes[j - 1]!);
    }
  }
  return newNodes[0]!;
}
