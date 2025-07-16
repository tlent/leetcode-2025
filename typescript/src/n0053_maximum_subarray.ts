export function maxSubarray(numbers: number[]): number {
  let max = numbers[0]!;
  let sum = numbers[0]!;

  for (const number of numbers.slice(1)) {
    if (sum < 0) {
      sum = 0;
    }
    sum += number;
    max = Math.max(sum, max);
  }

  return max;
}
