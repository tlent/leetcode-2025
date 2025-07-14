export function longestPalindrome(s: string): number {
  const counts: number[] = Array(52).fill(0);
  for (const c of s) {
    const index = toIndex(c);
    counts[index]! += 1;
  }
  let length = 0;
  for (const count of counts) {
    if (count % 2 === 0) {
      length += count;
    } else {
      length += count - 1;
    }
  }
  if (length < s.length) {
    length += 1;
  }
  return length;
}

function toIndex(c: string): number {
  const charCode = c.charCodeAt(0);
  if (97 <= charCode && charCode <= 122) {
    return c.charCodeAt(0) - 'a'.charCodeAt(0);
  } else if (65 <= charCode && charCode <= 90) {
    return 26 + c.charCodeAt(0) - 'A'.charCodeAt(0);
  } else {
    throw new Error('invalid character');
  }
}
