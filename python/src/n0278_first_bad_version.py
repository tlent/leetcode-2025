from typing import Callable


def first_bad_version(n: int, is_bad_version: Callable[[int], bool]) -> int:
    start = 0
    end = n
    while start < end:
        mid = (start + end) // 2
        if is_bad_version(mid):
            end = mid
        else:
            start = mid + 1
    return end
