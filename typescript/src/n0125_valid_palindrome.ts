export function isPalindrome(s: string): boolean {
  let start = 0;
  let end = s.length - 1;

  while (start < end) {
    const a = s[start]!;
    const b = s[end]!;
    if (!isAlphanumeric(a)) {
      start++;
    } else if (!isAlphanumeric(b)) {
      end--;
    } else if (a.toLowerCase() !== b.toLowerCase()) {
      return false;
    } else {
      start++;
      end--;
    }
  }
  return true;
}

function isAlphanumeric(character: string): boolean {
  return (
    ('a' <= character && character <= 'z') ||
    ('A' <= character && character <= 'Z') ||
    ('0' <= character && character <= '9')
  );
}
