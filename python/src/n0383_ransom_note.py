from collections import Counter


def can_construct(ransom_note: str, magazine: str) -> bool:
    return Counter(ransom_note) < Counter(magazine)
