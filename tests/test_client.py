import netcode


def test_unconnected_client():
    address = ("0.0.0.0", 1234)  # noqa: S104
    key = netcode.generate_key()
    token = netcode.ConnectToken(
        server_addresses=[address],
        protocol_id=0,
        client_id=0,
        private_key=key,
    )
    client = netcode.Client(token)
    assert not client.is_pending()
    assert not client.is_connected()
    assert client.is_disconnected()
    assert not client.is_error()
    assert client.addr()[0] == "0.0.0.0"  # noqa: S104
    assert client.address()[0] == "0.0.0.0"  # noqa: S104
    assert client.recv() is None
    client.send(b"hello")
    assert client.recv() is None
    client.disconnect()
    assert client.is_disconnected()
    assert not client.is_connected()
    assert not client.is_pending()
    assert not client.is_error()
    assert client.recv() is None
