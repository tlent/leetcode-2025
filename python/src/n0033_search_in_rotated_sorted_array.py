def search(nums: list[int], target: int) -> int:
    last = nums[-1]
    start = 0
    end = len(nums) - 1
    while start <= end:
        mid = (start + end) // 2
        if nums[mid] == target:
            return mid
        elif target <= last and nums[mid] > last:
            start = mid + 1
        elif target > last and nums[mid] <= last:
            end = mid - 1
        elif nums[mid] < target:
            start = mid + 1
        else:
            end = mid - 1
    return -1
