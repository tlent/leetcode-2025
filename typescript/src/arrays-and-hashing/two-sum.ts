function twoSum(numbers: number[], target: number): number[] {
  const seen = new Map<number, number>();
  for (const [index, number] of numbers.entries()) {
    const complement = target - number;
    const complementIndex = seen.get(complement);
    if (complementIndex !== undefined) return [complementIndex, index];
    seen.set(number, index);
  }
  throw new Error("no solution found");
}

function twoSumSort(numbers: number[], target: number): number[] {
  const values = numbers
    .map((number, index) => ({ number, index }))
    .sort((a, b) => a.number - b.number);
  let start = 0;
  let end = values.length - 1;
  while (start < end) {
    // Non-null assertions are safe because 0 <= start < end < values.length
    const startValue = values[start]!; // eslint-disable-line @typescript-eslint/no-non-null-assertion
    const endValue = values[end]!; // eslint-disable-line @typescript-eslint/no-non-null-assertion
    const sum = startValue.number + endValue.number;
    if (sum === target) return [startValue.index, endValue.index];
    if (sum < target) start += 1;
    if (sum > target) end -= 1;
  }
  throw new Error("no solution found");
}

if (import.meta.vitest) {
  const { test, expect } = await import("vitest");

  test("twoSum", () => {
    expect(twoSum([2, 7, 11, 15], 9)).toEqual([0, 1]);
    expect(twoSum([3, 2, 4], 6)).toEqual([1, 2]);
    expect(twoSum([3, 3], 6)).toEqual([0, 1]);
  });

  test("twoSumSort", () => {
    expect(twoSumSort([2, 7, 11, 15], 9).sort()).toEqual([0, 1]);
    expect(twoSumSort([3, 2, 4], 6).sort()).toEqual([1, 2]);
    expect(twoSumSort([3, 3], 6).sort()).toEqual([0, 1]);
  });
}
