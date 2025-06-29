def two_sum(numbers: list[int], target: int) -> list[int]:
    seen: dict[int, int] = {}
    for index, number in enumerate(numbers):
        complement = target - number
        if complement in seen:
            return [seen[complement], index]
        seen[number] = index
    raise ValueError("no solution found")


