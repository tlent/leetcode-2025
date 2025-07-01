def binary_search(numbers: list[int], target: int) -> int:
    start = 0
    end = len(numbers) - 1
    while start <= end:
        mid = (start + end) // 2
        if numbers[mid] == target:
            return mid
        elif numbers[mid] < target:
            start = mid + 1
        else:
            end = mid - 1
    return -1
