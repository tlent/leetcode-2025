def max_subarray(numbers: list[int]) -> int:
    total = numbers[0]
    max_total = numbers[0]

    for number in numbers[1:]:
        if total < 0:
            total = 0
        total += number
        max_total = max(max_total, total)

    return max_total
