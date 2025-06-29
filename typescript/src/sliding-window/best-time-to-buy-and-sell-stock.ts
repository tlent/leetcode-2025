export function maxProfit(prices: number[]): number {
  let minPrice = prices[0]!;
  let maxProfit = 0;
  for (let i = 1; i < prices.length; i++) {
    const price = prices[i]!;
    maxProfit = Math.max(maxProfit, price - minPrice);
    minPrice = Math.min(minPrice, price);
  }
  return maxProfit;
}

