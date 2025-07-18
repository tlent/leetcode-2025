from heapq import heappush, heappushpop


def _distance(point: list[int]) -> int:
    return point[0] ** 2 + point[1] ** 2


# O(n log n)
def k_closest(points: list[list[int]], k: int) -> list[list[int]]:
    points.sort(key=_distance)
    return points[:k]


# O(n log k)
def k_closest_heap(points: list[list[int]], k: int) -> list[list[int]]:
    heap: list[tuple[int, list[int]]] = []
    for point in points:
        item = (-_distance(point), point)
        if len(heap) < k:
            heappush(heap, item)
        else:
            heappushpop(heap, item)
    return [point for [_, point] in heap]
