export function binarySearch(numbers: number[], target: number): number {
  let start = 0;
  let end = numbers.length;
  while (start < end) {
    const mid = Math.floor((start + end) / 2);
    if (numbers[mid] === target) {
      return mid;
    } else if (numbers[mid]! < target) {
      start = mid + 1;
    } else {
      end = mid;
    }
  }
  return -1;
}
