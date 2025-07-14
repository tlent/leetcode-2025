export function majorityElement(numbers: number[]): number {
  let element = 0;
  let count = 0;
  for (const number of numbers) {
    if (count === 0) {
      element = number;
    }
    count += element === number ? 1 : -1;
  }
  return element;
}
