export function insert(
  intervals: number[][],
  newInterval: number[],
): number[][] {
  let start = intervals.findIndex(
    (interval) => interval[1]! >= newInterval[0]!,
  );
  if (start === -1) {
    start = intervals.length;
  }
  let end = intervals.findIndex((interval) => interval[0]! > newInterval[1]!);
  if (end === -1) {
    end = intervals.length;
  }

  if (start < end) {
    newInterval[0] = Math.min(newInterval[0]!, intervals[start]![0]!);
    newInterval[1] = Math.max(newInterval[1]!, intervals[end - 1]![1]!);
  }

  return [...intervals.slice(0, start), newInterval, ...intervals.slice(end)];
}
