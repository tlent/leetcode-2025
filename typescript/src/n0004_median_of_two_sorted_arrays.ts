export function findMedian(a: number[], b: number[]): number {
  if (a.length > b.length) {
    [a, b] = [b, a];
  }
  const m = a.length;
  const n = b.length;
  const length = m + n;
  const half = Math.floor(length / 2);
  let start = 0;
  let end = m;
  while (true) {
    const i = Math.floor((start + end) / 2);
    const j = half - i;

    const aLeft = i > 0 ? a[i - 1] : Number.NEGATIVE_INFINITY;
    const aRight = i < m ? a[i] : Number.POSITIVE_INFINITY;
    const bLeft = j > 0 ? b[j - 1] : Number.NEGATIVE_INFINITY;
    const bRight = j < n ? b[j] : Number.POSITIVE_INFINITY;

    if (aLeft <= bRight && bLeft <= aRight) {
      if (length % 2 == 0) {
        return (Math.max(aLeft, bLeft) + Math.min(aRight, bRight)) / 2;
      }
      return Math.min(aRight, bRight);
    } else if (aLeft > bRight) {
      end = i - 1;
    } else {
      start = i + 1;
    }
  }
}
