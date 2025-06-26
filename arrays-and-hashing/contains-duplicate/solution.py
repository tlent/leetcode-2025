def has_duplicate(numbers: list[int]) -> bool:
    return len(set(numbers)) < len(numbers)
