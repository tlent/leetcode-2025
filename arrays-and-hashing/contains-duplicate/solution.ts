function hasDuplicate(numbers: number[]): boolean {
  return (new Set(numbers)).size < numbers.length;
}
