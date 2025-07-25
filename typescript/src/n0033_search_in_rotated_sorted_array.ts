export function search(nums: number[], target: number): number {
  const last = nums.at(-1)!;
  let start = 0;
  let end = nums.length - 1;
  while (start <= end) {
    const mid = Math.floor((start + end) / 2);
    if (nums[mid] === target) {
      return mid;
    } else if (target <= last && nums[mid] > last) {
      start = mid + 1;
    } else if (target > last && nums[mid] <= last) {
      end = mid - 1;
    } else if (nums[mid] < target) {
      start = mid + 1;
    } else {
      end = mid - 1;
    }
  }
  return -1;
}
