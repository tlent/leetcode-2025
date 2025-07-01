from n0733_flood_fill import flood_fill


def test_flood_fill() -> None:
    assert flood_fill([[1, 1, 1], [1, 1, 0], [1, 0, 1]], 1, 1, 2) == [
        [2, 2, 2],
        [2, 2, 0],
        [2, 0, 1],
    ]
    assert flood_fill([[0, 0, 0], [0, 0, 0]], 0, 0, 0) == [[0, 0, 0], [0, 0, 0]]
