function isAnagram(s: string, t: string): boolean {
  if (s.length !== t.length) return false;
  const sCounts = countCharacters(s);
  const tCounts = countCharacters(t);
  return arrayEquals(sCounts, tCounts);
}

function countCharacters(s: string): number[] {
  const counts = new Array<number>(26).fill(0);
  for (const c of s) {
    const index = c.charCodeAt(0) - 97;
    // Non-null assertion is safe: c is guaranteed to be a lowercase letter so 0 <= index < 26
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    counts[index]!++;
  }
  return counts;
}

function arrayEquals<Type>(a: Type[], b: Type[]): boolean {
  for (let i = 0; i < a.length; i++) {
    if (a[i] !== b[i]) return false;
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
