import netcode
from netcode import client_state


def test_server_no_clients():
    listen_address = ("0.0.0.0", 5555)  # noqa: S104
    protocol_id = 0xDEADBEEF
    private_key = netcode.generate_key()

    server = netcode.Server(listen_address, protocol_id, private_key)
    assert server

    assert server.recv() is None
    assert server.num_connected_clients() == 0

    server.send(b"hello", 0)
