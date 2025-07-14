from collections import Counter


def longest_palindrome(s: str) -> int:
    counts = Counter(s)
    length = 0
    for count in counts.values():
        if count % 2 == 0:
            length += count
        else:
            length += count - 1
    if length < len(s):
        length += 1
    return length
