function isPalindrome(s: string): boolean {
  let start = 0;
  let end = s.length - 1;
  
  while (start < end) {
    const startCode = s.charCodeAt(start);
    const endCode = s.charCodeAt(end);
    
    if (!isAlphanumeric(startCode)) {
      start++;
    } else if (!isAlphanumeric(endCode)) {
      end--;
    } else if (toLowerCase(startCode) !== toLowerCase(endCode)) {
      return false;
    } else {
      start++;
      end--;
    }
  }
  return true;
}

function isAlphanumeric(charCode: number): boolean {
  return (charCode >= 48 && charCode <= 57) || // 0-9
         (charCode >= 65 && charCode <= 90) || // A-Z
         (charCode >= 97 && charCode <= 122);  // a-z
}

function toLowerCase(charCode: number): number {
  return charCode >= 65 && charCode <= 90 ? charCode + 32 : charCode;
}
