from n0067_add_binary import add_binary


def test_add_binary() -> None:
    assert add_binary("11", "1") == "100"
    assert add_binary("1010", "1011") == "10101"
