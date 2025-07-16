def insert(intervals: list[list[int]], new_interval: list[int]) -> list[list[int]]:
    start = next(
        (i for i, interval in enumerate(intervals) if interval[1] >= new_interval[0]),
        len(intervals),
    )
    end = next(
        (i for i, interval in enumerate(intervals) if interval[0] > new_interval[1]),
        len(intervals),
    )

    if start < end:
        new_interval[0] = min(new_interval[0], intervals[start][0])
        new_interval[1] = max(new_interval[1], intervals[end - 1][1])

    return intervals[:start] + [new_interval] + intervals[end:]
