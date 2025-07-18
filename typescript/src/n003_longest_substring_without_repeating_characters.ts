export function lengthOfLongestSubstring(s: string): number {
  const seen: (number | null)[] = Array(256).fill(null);
  let maxLength = 0;
  let start = 0;
  for (let i = 0; i < s.length; i++) {
    const c = s.charCodeAt(i);
    if (seen[c] !== null) {
      start = Math.max(start, seen[c]! + 1);
    }
    maxLength = Math.max(maxLength, i - start + 1);
    seen[c] = i;
  }
  return maxLength;
}
