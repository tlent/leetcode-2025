fn max_profit(prices: &[i32]) -> i32 {
    let mut min_price = prices[0];
    let mut max_profit = 0;
    for &price in &prices[1..] {
        max_profit = max_profit.max(price - min_price);
        min_price = min_price.min(price);
    }
    max_profit
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_max_profit() {
        assert_eq!(max_profit(&[7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(max_profit(&[7, 6, 4, 3, 1]), 0);
    }
}
