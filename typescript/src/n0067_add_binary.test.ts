import { expect } from "jsr:@std/expect";

import { addBinary } from "./n0067_add_binary.ts";

Deno.test("addBinary", () => {
  expect(addBinary("11", "1")).toEqual("100");
  expect(addBinary("1010", "1011")).toEqual("10101");
});
