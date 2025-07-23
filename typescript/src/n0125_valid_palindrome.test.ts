import { expect } from "jsr:@std/expect";
import { isPalindrome } from "./n0125_valid_palindrome.ts";

Deno.test("isPalindrome", () => {
  expect(isPalindrome("A man, a plan, a canal: Panama")).toBe(true);
  expect(isPalindrome("race a car")).toBe(false);
  expect(isPalindrome(" ")).toBe(true);
});
