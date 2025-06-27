function isPalindrome(s: string): boolean {
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
    ("a" <= character && character <= "z") ||
    ("A" <= character && character <= "Z") ||
    ("0" <= character && character <= "9")
  );
}

if (import.meta.vitest) {
  const { test, expect } = await import("vitest");

  test("isPalindrome", () => {
    expect(isPalindrome("A man, a plan, a canal: Panama")).toBe(true);
    expect(isPalindrome("race a car")).toBe(false);
    expect(isPalindrome(" ")).toBe(true);
  });
}
