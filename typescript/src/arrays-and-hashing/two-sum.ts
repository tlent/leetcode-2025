function twoSum(numbers: number[], target: number): number[] {
  const seen = new Map();
  for (const [index, number] of numbers.entries()) {
    const complement = target - number;
    if (seen.has(complement)) return [seen.get(complement), index];
    seen.set(number, index);
  }
  throw new Error("no solution found");
};

function twoSumSort(numbers: number[], target: number): number[] {
  const pairs = numbers.map((value, index) => ({value, index})).sort((a, b) => a.value - b.value);
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
