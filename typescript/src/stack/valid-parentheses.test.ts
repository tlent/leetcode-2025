import { test, expect } from "vitest";
import { valid_parentheses } from "./valid-parentheses";

test("isValid", () => {
  expect(valid_parentheses("()")).toBe(true);
  expect(valid_parentheses("()[]{}")).toBe(true);
  expect(valid_parentheses("(]")).toBe(false);
  expect(valid_parentheses("([])")).toBe(true);
});
