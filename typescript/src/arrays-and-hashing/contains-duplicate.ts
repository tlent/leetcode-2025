function hasDuplicate(numbers: number[]): boolean {
  return (new Set(numbers)).size < numbers.length;
}

function hasDuplicateEarlyReturn(numbers: number[]): boolean {
  const seen = new Set();
  for (const number of numbers) {
    if (seen.has(number)) return true;
    seen.add(number);
  }
  return false;
}

if (import.meta.vitest) {
  const { test, expect } = await import('vitest');
  
  test('hasDuplicate', () => {
    expect(hasDuplicate([1, 2, 3, 1])).toBe(true);
    expect(hasDuplicate([1, 2, 3, 4])).toBe(false);
    expect(hasDuplicate([1, 1, 1, 3, 3, 4, 3, 2, 4, 2])).toBe(true);
  });
  
  test('hasDuplicateEarlyReturn', () => {
    expect(hasDuplicateEarlyReturn([1, 2, 3, 1])).toBe(true);
    expect(hasDuplicateEarlyReturn([1, 2, 3, 4])).toBe(false);
    expect(hasDuplicateEarlyReturn([1, 1, 1, 3, 3, 4, 3, 2, 4, 2])).toBe(true);
  });
}
