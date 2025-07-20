def three_sum(numbers: list[int]) -> list[list[int]]:
    numbers.sort()
    result: list[list[int]] = []
    for i in range(len(numbers)):
        if i > 0 and numbers[i - 1] == numbers[i]:
            continue
        start = i + 1
        end = len(numbers) - 1
        while start < end:
            total = numbers[i] + numbers[start] + numbers[end]
            if total < 0:
                start += 1
            elif total > 0:
                end -= 1
            else:
                result.append([numbers[i], numbers[start], numbers[end]])
                start += 1
                while start < end and numbers[start - 1] == numbers[start]:
                    start += 1
    return result
