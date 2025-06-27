def max_profit(prices: list[int]) -> int:
    min_price = prices[0]
    max_profit = 0
    for price in prices[1:]:
        max_profit = max(max_profit, price - min_price)
        min_price = min(min_price, price)
    return max_profit

def test_max_profit() -> None:
    assert max_profit([7,1,5,3,6,4]) == 5
    assert max_profit([7,6,4,3,1]) == 0
