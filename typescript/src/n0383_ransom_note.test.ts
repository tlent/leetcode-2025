import { expect } from "jsr:@std/expect";
import { canConstruct } from "./n0383_ransom_note.ts";

Deno.test("canConstruct", () => {
  expect(canConstruct("a", "b")).toBe(false);
  expect(canConstruct("aa", "ab")).toBe(false);
  expect(canConstruct("aa", "aab")).toBe(true);
});
