export function isAnagram(s: string, t: string): boolean {
  if (s.length !== t.length) return false;

  const counts = new Array<number>(26).fill(0);
  for (const c of s) {
    counts[c.charCodeAt(0) - 97]!++;
  }
  for (const c of t) {
    const index = c.charCodeAt(0) - 97;
    if (counts[index] === 0) return false;
    counts[index]!--;
  }

  return true;
}
