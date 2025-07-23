import { expect } from "jsr:@std/expect";

import { longestPalindrome } from "./n0409_longest_palindrome.ts";

Deno.test("longestPalindrome", () => {
  expect(longestPalindrome("abccccdd")).toEqual(7);
  expect(longestPalindrome("a")).toEqual(1);
});
