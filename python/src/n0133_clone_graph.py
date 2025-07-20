from typing import Optional
from utils.graph import Node


def clone_graph(node: Optional[Node]) -> Optional[Node]:
    if not node:
        return None
    cloned: dict[int, Node] = dict()
    stack = [node]
    while stack:
        original = stack.pop()
        if original.value not in cloned:
            cloned[original.value] = Node(original.value)
        clone = cloned[original.value]
        for neighbor in original.neighbors:
            if neighbor.value not in cloned:
                cloned[neighbor.value] = Node(neighbor.value)
                stack.append(neighbor)
            clone.neighbors.append(cloned[neighbor.value])
    return cloned[node.value]


def clone_graph_recursive(node: Optional[Node]) -> Optional[Node]:
    if not node:
        return None
    cloned: dict[int, Node] = {}

    def dfs(original: Node) -> Node:
        if original.value in cloned:
            return cloned[original.value]
        clone = Node(original.value)
        cloned[original.value] = clone
        clone.neighbors = [dfs(neighbor) for neighbor in original.neighbors]
        return clone

    return dfs(node)
