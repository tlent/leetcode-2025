def has_duplicate(numbers: list[int]) -> bool:
    return len(set(numbers)) < len(numbers)


def has_duplicate_early_return(numbers: list[int]) -> bool:
    seen: set[int] = set()
    for number in numbers:
        if number in seen:
            return True
        seen.add(number)
    return False
