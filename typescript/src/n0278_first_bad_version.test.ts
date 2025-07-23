import { expect } from "jsr:@std/expect";

import { firstBadVersion } from "./n0278_first_bad_version.ts";

Deno.test("firstBadVersion example one", () => {
  expect(firstBadVersion(5, (v) => v >= 4)).toEqual(4);
});
Deno.test("firstBadVersion example two", () => {
  expect(firstBadVersion(1, (v) => v >= 1)).toEqual(1);
});
