pub fn max_profit(prices: &[i32]) -> i32 {
    let mut min_price = prices[0];
    let mut max_profit = 0;
    for &price in &prices[1..] {
        max_profit = max_profit.max(price - min_price);
        min_price = min_price.min(price);
    }
    max_profit
}
