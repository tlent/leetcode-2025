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
