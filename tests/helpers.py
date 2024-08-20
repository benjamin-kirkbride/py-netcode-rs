"""Helper functions for the tests."""

import logging
import random
import time

import netcode
from tests import parallel

logger = logging.getLogger(__name__)


class ServerProcess(parallel.SafeProcess):
    """Process that runs a server."""

    use_stop_event = True

    def __init__(
        self,
        name: str = "Server Process",
        listen_address: netcode.Address = ("127.0.0.1", 0),  # noqa: S104
        protocol_id: int = 0xDEADBEEF,
        private_key: bytes | None = None,
        update_interval: float = 1 / 60,
    ) -> None:
        """Init."""
        super().__init__(name=name)

        if private_key is None:
            private_key = netcode.generate_key()
        self.server = netcode.Server(listen_address, protocol_id, private_key)
        self.update_interval = update_interval

    def __enter__(self) -> "ServerProcess":
        """Enter the context manager."""
        self.start()
        return self

    def __exit__(self, *args: object) -> None:
        """Exit the context manager."""
        self.stop()

    def user_target(self) -> None:
        """Run the server."""
        logger.info(f"'{self.name}' started")

        assert self._stop_event is not None

        start_time = time.monotonic()
        while not self._stop_event.is_set():
            elapsed_time = time.monotonic() - start_time
            # self.server.update(elapsed_time)

            # FIXME: deduct the time spent updating and receiving
            time.sleep(self.update_interval)

        logger.info(f"'{self.name}' stopped")


class ClientProcess(parallel.SafeProcess):
    """Process that runs a client."""

    use_stop_event = True

    def __init__(
        self,
        server: netcode.Server,
        client_id: netcode.ClientID | None = None,
        name: str = "Client Process",
        update_interval: float = 1 / 60,
    ) -> None:
        """Init."""
        super().__init__(name=name)

        client_id = client_id or random.getrandbits(64)

        self.client = netcode.Client(server.token(client_id))
        self.update_interval = update_interval

    def user_target(self) -> None:
        """Run the client."""
        logger.info(f"'{self.name}' started")

        assert self._stop_event is not None

        start_time = time.monotonic()
        while not self._stop_event.is_set():
            elapsed_time = time.monotonic() - start_time
            self.client.update(elapsed_time)
            # while True:
            #     data = self.client.recv()
            #     if data is None:
            #         break
            #     self.result_queue.put((data, elapsed_time))
            time.sleep(self.update_interval)

        logger.info(f"'{self.name}' stopped")


class Clients:
    """Context manager for a list of clients."""

    def __init__(self, server: netcode.Server, initial_quantity: int = 0) -> None:
        """Init."""
        self.server = server
        self.clients: list[ClientProcess] = []
        for _ in range(initial_quantity):
            self.add()

    def __enter__(self) -> list[ClientProcess]:
        return self.clients

    def __exit__(self, *args: object) -> None:
        for client in self.clients:
            client.stop()
        self.clients.clear()

    def add(
        self,
        client_name: str | None = None,
        client_id: netcode.ClientID | None = None,
    ) -> ClientProcess:
        """Add a client."""
        client_name = client_name or f"Client {len(self.clients)}"
        client_process = ClientProcess(
            server=self.server,
            client_id=client_id,
            name=client_name,
        )
        client_process.start()
        client_process.client.connect()

        self.clients.append(client_process)
        return client_process
