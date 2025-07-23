import { expect } from "jsr:@std/expect";
import { binarySearch } from "./n0704_binary_search.ts";

Deno.test("binarySearch", () => {
  expect(binarySearch([-1, 0, 3, 5, 9, 12], 9)).toEqual(4);
  expect(binarySearch([-1, 0, 3, 5, 9, 12], 2)).toEqual(-1);
});
