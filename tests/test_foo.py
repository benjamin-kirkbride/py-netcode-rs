import netcode


def test_foo():
    assert netcode.sum_as_string(1, 2) == "3"


def test_generate_key():
    key = netcode.generate_key()
    print(key)
    print(type(key))
    assert len(key) == 32
    assert False
