import logging
from typing import TypeAlias

# logging needs to be configured before importing the netcode module
logging.getLogger().setLevel(logging.INFO)

# ruff: noqa: E402

from . import client_state
from .netcode import (
    CONNECT_TOKEN_BYTES,
    MAX_PACKET_SIZE,
    NETCODE_VERSION,
    PRIVATE_KEY_BYTES,
    USER_DATA_BYTES,
    Client,
    ClientIndex,
    ConnectToken,
    Server,
    generate_key,
)

Address: TypeAlias = tuple[str, int]
ClientID: TypeAlias = int

__all__ = [
    "CONNECT_TOKEN_BYTES",
    "MAX_PACKET_SIZE",
    "NETCODE_VERSION",
    "PRIVATE_KEY_BYTES",
    "USER_DATA_BYTES",
    "Client",
    "ConnectToken",
    "client_state",
    "generate_key",
    "Server",
    "ClientIndex",
    "Address",
    "ClientID",
    "my_module",
]
