import { PriorityQueue } from '@datastructures-js/priority-queue';

const distance = ([x, y]: [number, number]) => x * x + y * y;

// O(n log n)
export function kClosest(
  points: [number, number][],
  k: number
): [number, number][] {
  points.sort((a, b) => distance(a) - distance(b));
  return points.slice(0, k);
}

// O(n log k)
export function kClosestHeap(
  points: [number, number][],
  k: number
): [number, number][] {
  const heap = new PriorityQueue<[number, number]>((a, b) => {
    return distance(b) - distance(a);
  });
  for (const point of points) {
    heap.push(point);
    if (heap.size() > k) {
      heap.pop();
    }
  }
  return heap.toArray();
}
