export function climbStairs(n: number): number {
  let a = 1;
  let b = 1;
  for (let i = 2; i <= n; i++) {
    const sum = a + b;
    a = b;
    b = sum;
  }
  return b;
}
