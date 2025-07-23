import { expect } from "jsr:@std/expect";
import { valid_parentheses } from "./n0020_valid_parentheses.ts";

Deno.test("isValid", () => {
  expect(valid_parentheses("()")).toBe(true);
  expect(valid_parentheses("()[]{}")).toBe(true);
  expect(valid_parentheses("(]")).toBe(false);
  expect(valid_parentheses("([])")).toBe(true);
});
