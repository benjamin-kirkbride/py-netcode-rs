import time

import pytest

import netcode
from netcode import client_state
from tests import helpers


def test_server_no_clients():
    listen_address = ("0.0.0.0", 5555)  # noqa: S104
    protocol_id = 0xDEADBEEF
    private_key = netcode.generate_key()

    server = netcode.Server(listen_address, protocol_id, private_key)
    assert server

    assert server.recv() is None
    assert server.num_connected_clients() == 0
    assert server.clients == []


def test_client_server():
    server = netcode.Server(("127.0.0.1", 0), 0xDEADBEEF, netcode.generate_key())
    client = netcode.Client(server.token(123))
    client.connect()

    update_rate = 1 / 60

    start_time = time.monotonic()

    # give the server and client some time to connect
    # TODO: replace with a better way to wait for the connection
    for _ in range(10):
        elapsed_time = time.monotonic() - start_time
        server.update(elapsed_time)
        client.update(elapsed_time)
        time.sleep(update_rate)

    assert client.state() == client_state.CONNECTED

    client.send(b"Hello, server!")

    server_received = False
    client_received = False
    for _ in range(100):
        if server_received and client_received:
            return
        elapsed_time = time.monotonic() - start_time
        server.update(elapsed_time)
        client.update(elapsed_time)

        while True:
            assert server.num_connected_clients() == 1
            server_result = server.recv()
            if server_result is None:
                break
            server_received = True
            server_data, client_index = server_result
            print(f"Server received: {server_data!r}")
            server.send(b"Hello, client!", client_index)

        while True:
            assert client.state() == client_state.CONNECTED
            client_data = client.recv()
            if client_data is None:
                break
            client_received = True
            print(f"Client received: {client_data!r}")

        time.sleep(update_rate)

    pytest.fail("Did not receive expected messages")


def test_client_server_process():

    with (
        helpers.ServerProcess() as server_process,
        helpers.Clients(server_process.server, 1) as client_processes,
    ):
        time.sleep(0.1)
        client = client_processes[0].client
        server = server_process.server

        update_rate = 1 / 60
        start_time = time.monotonic()

        for _ in range(10):
            elapsed_time = time.monotonic() - start_time
            server.update(elapsed_time)
            # client.update(elapsed_time)
            time.sleep(update_rate)

        assert server.num_connected_clients() == 1
        assert client.state() == client_state.CONNECTED

        client.send(b"Hello, world!")
        for _ in range(10):
            print(client.recv())
            print(server.recv())
            time.sleep(0.1)

        assert server.num_connected_clients() == 1
        assert client.state() == client_state.CONNECTED
