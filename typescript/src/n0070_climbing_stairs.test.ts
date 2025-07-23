import { expect } from "jsr:@std/expect";
import { climbStairs } from "./n0070_climbing_stairs.ts";

Deno.test("climbStairs", () => {
  expect(climbStairs(2)).toEqual(2);
  expect(climbStairs(3)).toEqual(3);
});
