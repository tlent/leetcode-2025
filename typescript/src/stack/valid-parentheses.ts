function valid_parentheses(s: string): boolean {
  const stack: string[] = [];
  for (const c of s) {
    if (c === "(") {
      stack.push(")");
    } else if (c === "{") {
      stack.push("}");
    } else if (c === "[") {
      stack.push("]");
    } else if (stack.pop() !== c) {
      return false;
    }
  }
  return stack.length === 0;
}

if (import.meta.vitest) {
  const { test, expect } = await import("vitest");

  test("isValid", () => {
    expect(valid_parentheses("()")).toBe(true);
    expect(valid_parentheses("()[]{}")).toBe(true);
    expect(valid_parentheses("(]")).toBe(false);
    expect(valid_parentheses("([])")).toBe(true);
  });
}
