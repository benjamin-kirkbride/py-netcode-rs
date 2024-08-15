<!-- TODO: use cog to insert the examples, and test them -->

<div align="center">
  <h1>
    <a href="https://github.com/mas-bandwidth/netcode"><code>netcode</code></a> Python Bindings
  </h1>
</div>

## Protocol

This package implements the [netcode](https://github.com/mas-bandwidth/netcode)
network protocol created by [Glenn Fiedler](https://mas-bandwidth.com/) by wrapping [benny-n's Rust implementation](https://github.com/benny-n/netcode).

> `netcode` is a simple connection based client/server protocol built on top of UDP.

See the [specification](STANDARD.md) to learn more about how the protocol works.

## Install

<!-- TODO -->

**NOTE:** While the package name is `py-netcode-rs`, the library name is actually `netcode`.

## Examples

### server

```python
import time

import netcode

listen_address = ("0.0.0.0", 5555) # the address to listen on
protocol_id = 0xdeadbeef # a unique number that must match the client's protocol id
private_key = netcode.generate_key() # you can also provide your own key

server = netcode.Server(listen_address, protocol_id, private_key)

# Run the server at 60Hz
start = time.monotonic()
tick_rate = 1/60
while True:
    elapsed = time.monotonic() - start
    server.update(elapsed)
    while True:
        result = server.recv()
        if result is None:
            break
        packet, client_id = result
        print(f"Received packet from client {client_id}: {packet}")

    time.sleep(tick_rate)
```

### client

```python
import time

import netcode

# Generate a connection token for the client
server_address = ("0.0.0.0", 5555) # the address of the server (can be a list of addresses as well)
protocol_id = 0xdeadbeef # a unique number that must match the server's protocol id
client_id = netcode.generate_key() # an 64 bit number that must be unique for each client
private_key = netcode.generate_key() # you can also provide your own key
connect_token = netcode.ConnectToken(server_address, protocol_id, client_id, private_key)

# Start the client
client = netcode.Client(connect_token)
client.connect()

# run the client at 60Hz
start = time.monotonic()
tick_rate = 1/60
while True:
    elapsed = time.monotonic() - start
    client.update(elapsed)
    while True:
        packet = client.recv()
        if packet is None:
            break
        print(f"Received packet from server: {packet}")

    time.sleep(tick_rate)
```
