export function canConstruct(ransom_note: string, magazine: string): boolean {
  const counts: number[] = Array(26).fill(0);
  for (const c of magazine) {
    const index = c.charCodeAt(0) - 97;
    counts[index]! += 1;
  }
  for (const c of ransom_note) {
    const index = c.charCodeAt(0) - 97;
    if (counts[index] === 0) {
      return false;
    }
    counts[index]! -= 1;
  }
  return true;
}
