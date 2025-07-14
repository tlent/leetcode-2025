def majority_element(numbers: list[int]) -> int:
    count = 0
    element = 0
    for number in numbers:
        if count == 0:
            element = number
        count += 1 if element == number else -1
    return element
