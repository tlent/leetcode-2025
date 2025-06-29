import { test, expect } from "vitest";
import { twoSum, twoSumSort } from "./two-sum";

test("twoSum", () => {
  expect(twoSum([2, 7, 11, 15], 9)).toEqual([0, 1]);
  expect(twoSum([3, 2, 4], 6)).toEqual([1, 2]);
  expect(twoSum([3, 3], 6)).toEqual([0, 1]);
});

test("twoSumSort", () => {
  expect(twoSumSort([2, 7, 11, 15], 9).sort()).toEqual([0, 1]);
  expect(twoSumSort([3, 2, 4], 6).sort()).toEqual([1, 2]);
  expect(twoSumSort([3, 3], 6).sort()).toEqual([0, 1]);
});
