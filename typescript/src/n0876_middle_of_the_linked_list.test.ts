import { expect } from "jsr:@std/expect";

import { middleNode } from "./n0876_middle_of_the_linked_list.ts";
import { List } from "./utils/linked_list.ts";

Deno.test("middleNode", () => {
  let list = new List([1, 2, 3, 4, 5]);
  expect(middleNode(list.head)?.value).toEqual(3);
  list = new List([1, 2, 3, 4, 5, 6]);
  expect(middleNode(list.head)?.value).toEqual(4);
});
