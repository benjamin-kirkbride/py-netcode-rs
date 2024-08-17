import netcode
from netcode import client_state


def test_client_state():
    assert client_state.CONNECT_TOKEN_EXPIRED == "CONNECT_TOKEN_EXPIRED"  # noqa: S105
    assert client_state.CONNECTION_TIMED_OUT == "CONNECTION_TIMED_OUT"
    assert client_state.CONNECTION_REQUEST_TIMED_OUT == "CONNECTION_REQUEST_TIMED_OUT"
    assert client_state.CHALLENGE_RESPONSE_TIMED_OUT == "CHALLENGE_RESPONSE_TIMED_OUT"
    assert client_state.CONNECTION_DENIED == "CONNECTION_DENIED"
    assert client_state.DISCONNECTED == "DISCONNECTED"
    assert client_state.SENDING_CONNECTION_REQUEST == "SENDING_CONNECTION_REQUEST"
    assert client_state.SENDING_CHALLENGE_RESPONSE == "SENDING_CHALLENGE_RESPONSE"
    assert client_state.CONNECTED == "CONNECTED"


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
    assert client.state() == client_state.DISCONNECTED
    assert client.state() != client_state.CONNECTED
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
