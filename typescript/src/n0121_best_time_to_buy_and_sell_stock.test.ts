import { expect } from "jsr:@std/expect";
import { maxProfit } from "./n0121_best_time_to_buy_and_sell_stock.ts";

Deno.test("maxProfit", () => {
  expect(maxProfit([7, 1, 5, 3, 6, 4])).toEqual(5);
  expect(maxProfit([7, 6, 4, 3, 1])).toEqual(0);
});
