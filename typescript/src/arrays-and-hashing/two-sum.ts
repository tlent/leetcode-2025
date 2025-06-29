export function twoSum(numbers: number[], target: number): number[] {
  const seen = new Map<number, number>();
  for (const [index, number] of numbers.entries()) {
    const complement = target - number;
    const complementIndex = seen.get(complement);
    if (complementIndex !== undefined) return [complementIndex, index];
    seen.set(number, index);
  }
  throw new Error("no solution found");
}

export function twoSumSort(numbers: number[], target: number): number[] {
  const values = numbers
    .map((number, index) => ({ number, index }))
    .sort((a, b) => a.number - b.number);
  let start = 0;
  let end = values.length - 1;
  while (start < end) {
    const startValue = values[start]!;
    const endValue = values[end]!;
    const sum = startValue.number + endValue.number;
    if (sum === target) return [startValue.index, endValue.index];
    else if (sum < target) start += 1;
    else end -= 1;
  }
  throw new Error("no solution found");
}

