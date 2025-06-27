function twoSum(numbers: number[], target: number): number[] {
  const seen = new Map();
  for (const [index, number] of numbers.entries()) {
    const complement = target - number;
    if (seen.has(complement)) return [seen.get(complement), index];
    seen.set(number, index);
  }
  throw new Error("no solution found");
}

function twoSumSort(numbers: number[], target: number): number[] {
  const pairs = numbers
    .map((value, index) => ({ value, index }))
    .sort((a, b) => a.value - b.value);
  let start = 0;
  let end = pairs.length - 1;
  while (start < end) {
    const sum = pairs[start]!.value + pairs[end]!.value;
    if (sum === target) return [pairs[start]!.index, pairs[end]!.index];
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
