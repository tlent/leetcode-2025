function isAnagram(s: string, t: string): boolean {
  if (s.length !== t.length) return false;

  const counts = new Array<number>(26).fill(0);
  for (const c of s) {
    // Safe: c is guaranteed to be lowercase letter, so 0 <= index < 26
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    counts[c.charCodeAt(0) - 97]!++;
  }
  for (const c of t) {
    const index = c.charCodeAt(0) - 97;
    if (counts[index] === 0) return false;
    // Safe: c is guaranteed to be lowercase letter, so 0 <= index < 26
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    counts[index]!--;
  }

  return true;
}

if (import.meta.vitest) {
  const { test, expect } = await import("vitest");

  test("isAnagram", () => {
    expect(isAnagram("anagram", "nagaram")).toBe(true);
    expect(isAnagram("rat", "car")).toBe(false);
    expect(isAnagram("listen", "silent")).toBe(true);
    expect(isAnagram("a", "ab")).toBe(false);
  });
}
