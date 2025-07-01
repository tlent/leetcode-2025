use crate::n0121_best_time_to_buy_and_sell_stock::*;

#[test]
fn test_max_profit() {
    assert_eq!(max_profit(&[7, 1, 5, 3, 6, 4]), 5);
    assert_eq!(max_profit(&[7, 6, 4, 3, 1]), 0);
}
