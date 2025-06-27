function isPalindrome(s: string): boolean {
  s = s.toLowerCase();
  let start = 0;
  let end = s.length - 1;
  while (start < end) {
    if (!isAlphanumeric(s.charCodeAt(start))) {
      start += 1;
    } else if (!isAlphanumeric(s.charCodeAt(end))) {
      end -= 1;
    } else if (s.charAt(start) === s.charAt(end)) {
      start += 1;
      end -= 1;
    } else {
      return false;
    }
  }
  return true;
}

function isAlphanumeric(charCode: number): boolean {
  return (charCode >= 97 && charCode <= 122) || (charCode >= 48 && charCode <= 57);
}
