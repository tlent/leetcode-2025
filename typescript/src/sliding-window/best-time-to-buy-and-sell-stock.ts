function maxProfit(prices: number[]): number {
  let minPrice = prices[0]!;
  let maxProfit = 0;
  for (let i = 1; i < prices.length; i++) {
    const price = prices[i]!;
    maxProfit = Math.max(maxProfit, price - minPrice);
    minPrice = Math.min(minPrice, price);
  }
  return maxProfit;
}

if (import.meta.vitest) {
  const { test, expect } = await import("vitest");

  test("maxProfit", () => {
    expect(maxProfit([7, 1, 5, 3, 6, 4])).toEqual(5);
    expect(maxProfit([7, 6, 4, 3, 1])).toEqual(0);
  });
}
