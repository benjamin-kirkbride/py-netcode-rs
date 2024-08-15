import netcode


def test_constants():
    assert netcode.CONNECT_TOKEN_BYTES == 2048  # noqa: PLR2004
    assert netcode.MAX_PACKET_SIZE == 1200  # noqa: PLR2004
    assert netcode.NETCODE_VERSION == b"NETCODE 1.02\x00"
    assert netcode.PRIVATE_KEY_BYTES == 32  # noqa: PLR2004
    assert netcode.USER_DATA_BYTES == 256  # noqa: PLR2004
