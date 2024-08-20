"""Python bindings for the netcode.io library."""

from typing import TypeAlias

from _netcode import (
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

__all__ = [
    "CONNECT_TOKEN_BYTES",
    "MAX_PACKET_SIZE",
    "NETCODE_VERSION",
    "PRIVATE_KEY_BYTES",
    "USER_DATA_BYTES",
    "Client",
    "ConnectToken",
    "generate_key",
    "Server",
    "ClientIndex",
]
