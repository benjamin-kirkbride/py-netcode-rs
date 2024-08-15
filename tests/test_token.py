import netcode


def test_generate_key():
    key = netcode.generate_key()
    assert len(key) == netcode.PRIVATE_KEY_BYTES


def test_token():
    key = netcode.generate_key()

    connect_token_1 = netcode.ConnectToken(
        server_addresses=[("localhost", 1234)],
        protocol_id=0,
        client_id=0,
        private_key=key,
    )
    assert connect_token_1
    assert bytes(connect_token_1)
    assert len(bytes(connect_token_1)) == netcode.CONNECT_TOKEN_BYTES

    connect_token_2 = netcode.ConnectToken(
        server_addresses=[("localhost", 1234)],
        protocol_id=0,
        client_id=0,
        private_key=key,
    )
    assert connect_token_2
    assert bytes(connect_token_2)
    assert len(bytes(connect_token_2)) == netcode.CONNECT_TOKEN_BYTES
    assert bytes(connect_token_1) != bytes(connect_token_2)
