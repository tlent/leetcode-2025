export class Node {
  value: number;
  neighbors: Node[];

  constructor(value: number) {
    this.value = value;
    this.neighbors = [];
  }
}

export class Graph {
  public node: Node | null;

  constructor(adjacency_lists: number[][]) {
    if (adjacency_lists.length === 0) {
      this.node = null;
    } else {
      const nodes = Array(adjacency_lists.length)
        .fill(null)
        .map((_, i) => new Node(i + 1));
      for (const [i, list] of adjacency_lists.entries()) {
        nodes[i]!.neighbors = list.map((j) => nodes[j - 1]!);
      }
      this.node = nodes[0]!;
    }
  }

  to_adjacency_lists(): number[][] {
    if (!this.node) {
      return [];
    }
    let maxValue = 0;
    const result = Array(100).fill(null);
    const visited = Array(100).fill(false);
    const stack = [this.node];
    while (stack.length > 0) {
      const node = stack.pop()!;
      maxValue = Math.max(maxValue, node.value);
      result[node.value - 1] = node.neighbors.map((n) => n.value);
      for (const neighbor of node.neighbors) {
        if (!visited[neighbor.value - 1]) {
          visited[neighbor.value - 1] = true;
          stack.push(neighbor);
        }
      }
    }
    return result.slice(0, maxValue);
  }
}
