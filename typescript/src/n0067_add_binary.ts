export function addBinary(a: string, b: string): string {
  const result = [];
  let carry = 0;
  let a_index = a.length - 1;
  let b_index = b.length - 1;
  while (a_index >= 0 || b_index >= 0 || carry > 0) {
    let sum = carry;
    if (a_index >= 0) {
      sum += Number(a[a_index]);
      a_index -= 1;
    }
    if (b_index >= 0) {
      sum += Number(b[b_index]);
      b_index -= 1;
    }
    carry = Math.floor(sum / 2);
    result.push((sum % 2).toString());
  }
  return result.reverse().join('');
}
