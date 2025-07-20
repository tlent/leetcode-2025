class Node:
    def __init__(self, value: int):
        self.value = value
        self.neighbors: list[Node] = []


class Graph:
    def __init__(self, adjacency_lists: list[list[int]]):
        if not adjacency_lists:
            self.node = None
        else:
            nodes = [Node(i + 1) for i in range(len(adjacency_lists))]
            for i, neighbors in enumerate(adjacency_lists):
                nodes[i].neighbors = [nodes[j - 1] for j in neighbors]
            self.node = nodes[0]

    def to_adjacency_lists(self) -> list[list[int]]:
        if not self.node:
            return []
        max_value = 0
        result: list[list[int]] = [[] for _ in range(100)]
        visited = [False] * 100
        stack = [self.node]
        while stack:
            node = stack.pop()
            max_value = max(max_value, node.value)
            result[node.value - 1] = [n.value for n in node.neighbors]
            for neighbor in node.neighbors:
                if not visited[neighbor.value - 1]:
                    visited[neighbor.value - 1] = True
                    stack.append(neighbor)
        return result[:max_value]
