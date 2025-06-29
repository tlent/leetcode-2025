export function valid_parentheses(s: string): boolean {
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
