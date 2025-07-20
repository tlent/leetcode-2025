export function threeSum(numbers: number[]): number[][] {
  numbers.sort((a, b) => a - b);
  const result = [];
  for (let i = 0; i < numbers.length; i++) {
    if (i > 0 && numbers[i - 1] === numbers[i]) {
      continue;
    }
    let start = i + 1;
    let end = numbers.length - 1;
    while (start < end) {
      const sum = numbers[i]! + numbers[start]! + numbers[end]!;
      if (sum < 0) {
        start += 1;
      } else if (sum > 0) {
        end -= 1;
      } else {
        result.push([numbers[i]!, numbers[start]!, numbers[end]!]);
        start += 1;
        while (start < end && numbers[start - 1] === numbers[start]) {
          start += 1;
        }
      }
    }
  }
  return result;
}
