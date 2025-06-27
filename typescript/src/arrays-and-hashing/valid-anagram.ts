function isAnagram(s: string, t: string): boolean {
  const sCounts = countCharacters(s);
  const tCounts = countCharacters(t);
  return arrayEquals(sCounts, tCounts);
}

function countCharacters(s: string): number[] {
  const counts = new Array(26).fill(0);
  for (const c of s) {
    counts[c.charCodeAt(0) - 97] += 1;
  }
  return counts;
}

function arrayEquals<Type>(a: Array<Type>, b: Array<Type>): boolean {
  for (let i = 0; i < a.length; i++) {
    if (a[i] !== b[i]) return false;
  }
  return true;
}

if (import.meta.vitest) {
  const { test, expect } = await import('vitest');
  
  test('isAnagram', () => {
    expect(isAnagram("anagram", "nagaram")).toBe(true);
    expect(isAnagram("rat", "car")).toBe(false);
    expect(isAnagram("listen", "silent")).toBe(true);
    expect(isAnagram("a", "ab")).toBe(false);
  });
}
