#![allow(non_camel_case_types)] // TODO: how to selectively apply this?

use ::netcode::{
    try_generate_key as x_try_generate_key, Client as x_Client, ClientConfig as x_ClientConfig,
    ClientIndex as x_ClientIndex, ClientState as x_ClientState, ConnectToken as x_ConnectToken,
    ConnectTokenBuilder as x_ConnectTokenBuilder, Error as x_Error, InvalidTokenError, Key,
    NetcodeSocket, Result as x_Result, Server as x_Server, ServerConfig as x_ServerConfig,
    Transceiver, CONNECT_TOKEN_BYTES as x_CONNECT_TOKEN_BYTES,
    MAX_PACKET_SIZE as x_MAX_PACKET_SIZE, NETCODE_VERSION as x_NETCODE_VERSION,
    PRIVATE_KEY_BYTES as x_PRIVATE_KEY_BYTES, USER_DATA_BYTES as x_USER_DATA_BYTES,
};
use pyo3::prelude::*;
use pyo3::types::*;
use std::fmt::{self, Debug};
use std::net::SocketAddr;
use std::net::ToSocketAddrs;

// type Key = [u8; x_PRIVATE_KEY_BYTES as usize];

#[pymodule]
mod _netcode {
    use pyo3::exceptions::PyRuntimeError;

    use super::*;

    #[pymodule_init]
    fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
        m.add("CONNECT_TOKEN_BYTES", x_CONNECT_TOKEN_BYTES)?;
        m.add("MAX_PACKET_SIZE", x_MAX_PACKET_SIZE)?;

        let netcode_version = PyBytes::new_bound(m.py(), x_NETCODE_VERSION);
        m.add("NETCODE_VERSION", netcode_version)?;

        m.add("PRIVATE_KEY_BYTES", x_PRIVATE_KEY_BYTES)?;
        m.add("USER_DATA_BYTES", x_USER_DATA_BYTES)?;
        Ok(())
    }

    #[pyfunction]
    fn generate_key(py: Python) -> PyObject {
        let key = x_try_generate_key()
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))
            .unwrap();
        PyBytes::new_bound(py, &key).into()
    }

    /// TODO: try_generate_key?

    #[pyclass]
    struct ConnectToken {
        inner: x_ConnectToken, // TODO: is this necessary?
        bytes: [u8; 2048],
    }

    #[pymethods]
    impl ConnectToken {
        #[new]
        fn new<'py>(
            server_addresses: &Bound<'py, PyList>,
            protocol_id: u64,
            client_id: u64,
            private_key: Key,
        ) -> PyResult<Self> {
            let tuple_addresses: Vec<(&str, u16)> = server_addresses
                .iter()
                .map(|item| -> PyResult<(&str, u16)> {
                    let tuple: &PyTuple = item.extract()?;
                    let host: &str = tuple.get_item(0)?.extract()?;
                    let port: u16 = tuple.get_item(1)?.extract()?;
                    Ok((host, port))
                })
                .collect::<PyResult<_>>()?;

            let addresses: Vec<SocketAddr> = tuple_addresses
                .iter()
                .filter_map(|(host, port)| {
                    format!("{}:{}", host, port)
                        .to_socket_addrs()
                        .ok()
                        .and_then(|mut addrs| addrs.next())
                })
                .collect();

            if addresses.is_empty() {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    "No valid socket addresses found",
                ));
            }

            let builder_1 =
                x_ConnectToken::build(&addresses[..], protocol_id, client_id, private_key);
            // TODO: handle optional args
            let inner = builder_1.generate().unwrap();

            let builder_2 =
                x_ConnectToken::build(&addresses[..], protocol_id, client_id, private_key);
            // TODO: handle optional args
            let to_become_bytes = builder_2.generate().unwrap();
            let bytes = to_become_bytes.try_into_bytes().unwrap();
            Ok(Self { inner, bytes })
        }

        fn __bytes__(&self, py: Python) -> PyObject {
            PyBytes::new_bound(py, &self.bytes).into()
        }
    }

    // this is a huge mess
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]

    enum ClientState {
        CONNECT_TOKEN_EXPIRED,
        CONNECTION_TIMED_OUT,
        CONNECTION_REQUEST_TIMED_OUT,
        CHALLENGE_RESPONSE_TIMED_OUT,
        CONNECTION_DENIED,
        DISCONNECTED,
        SENDING_CONNECTION_REQUEST,
        SENDING_CHALLENGE_RESPONSE,
        CONNECTED,
    }

    impl IntoPy<PyObject> for ClientState {
        fn into_py(self, py: Python) -> PyObject {
            self.to_string().into_py(py)
        }
    }

    impl fmt::Display for ClientState {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:?}", self)
            // or, alternatively:
            // fmt::Debug::fmt(self, f)
        }
    }

    impl From<x_ClientState> for ClientState {
        fn from(state: x_ClientState) -> Self {
            match state {
                x_ClientState::ConnectTokenExpired => Self::CONNECT_TOKEN_EXPIRED,
                x_ClientState::ConnectionTimedOut => Self::CONNECTION_TIMED_OUT,
                x_ClientState::ConnectionRequestTimedOut => Self::CONNECTION_REQUEST_TIMED_OUT,
                x_ClientState::ChallengeResponseTimedOut => Self::CHALLENGE_RESPONSE_TIMED_OUT,
                x_ClientState::ConnectionDenied => Self::CONNECTION_DENIED,
                x_ClientState::Disconnected => Self::DISCONNECTED,
                x_ClientState::SendingConnectionRequest => Self::SENDING_CONNECTION_REQUEST,
                x_ClientState::SendingChallengeResponse => Self::SENDING_CHALLENGE_RESPONSE,
                x_ClientState::Connected => Self::CONNECTED,
            }
        }
    }

    #[pymodule(submodule)]
    mod _client_state {
        use super::*;

        #[pymodule_init]
        fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
            m.add("CONNECT_TOKEN_EXPIRED", ClientState::CONNECT_TOKEN_EXPIRED)?;
            m.add("CONNECTION_TIMED_OUT", ClientState::CONNECTION_TIMED_OUT)?;
            m.add(
                "CONNECTION_REQUEST_TIMED_OUT",
                ClientState::CONNECTION_REQUEST_TIMED_OUT,
            )?;
            m.add(
                "CHALLENGE_RESPONSE_TIMED_OUT",
                ClientState::CHALLENGE_RESPONSE_TIMED_OUT,
            )?;
            m.add("CONNECTION_DENIED", ClientState::CONNECTION_DENIED)?;
            m.add("DISCONNECTED", ClientState::DISCONNECTED)?;
            m.add(
                "SENDING_CONNECTION_REQUEST",
                ClientState::SENDING_CONNECTION_REQUEST,
            )?;
            m.add(
                "SENDING_CHALLENGE_RESPONSE",
                ClientState::SENDING_CHALLENGE_RESPONSE,
            )?;
            m.add("CONNECTED", ClientState::CONNECTED)?;
            Ok(())
        }
    }

    #[pyclass]
    struct Client {
        inner: x_Client<NetcodeSocket>,
    }

    #[pymethods]
    impl Client {
        #[new]
        fn new<'py>(token: &mut ConnectToken) -> PyResult<Self> {
            let inner = x_Client::new(&token.bytes).unwrap();
            Ok(Self { inner })
        }

        // TODO: with_config

        // TODO: with_config_and_transceiver (is this even possible?)

        fn connect(&mut self) {
            self.inner.connect();
        }

        fn update(&mut self, time: f64) {
            self.inner.update(time);
        }

        fn recv(&mut self, py: Python<'_>) -> PyResult<Option<Py<PyBytes>>> {
            match self.inner.recv() {
                Some(data) => {
                    let py_bytes = PyBytes::new_bound(py, &data);
                    Ok(Some(py_bytes.into()))
                }
                None => Ok(None),
            }
        }

        fn send(&mut self, data: &[u8]) {
            self.inner.send(data).unwrap();
        }

        fn disconnect(&mut self) {
            self.inner.disconnect().unwrap();
        }

        fn address(&self) -> PyResult<(String, u16)> {
            let addr = self.inner.addr();
            Ok((addr.ip().to_string(), addr.port()))
        }

        fn addr(&self) -> PyResult<(String, u16)> {
            self.address()
        }

        fn state(&self) -> String {
            let x_state = self.inner.state();
            let state: ClientState = x_state.into();
            state.to_string()
        }

        fn is_error(&self) -> bool {
            self.inner.is_error()
        }

        fn is_pending(&self) -> bool {
            self.inner.is_pending()
        }

        fn is_connected(&self) -> bool {
            self.inner.is_connected()
        }

        fn is_disconnected(&self) -> bool {
            self.inner.is_disconnected()
        }
    }

    #[pyclass]
    struct ClientIndex {
        inner: x_ClientIndex,
    }

    #[pyclass]
    struct Server {
        inner: x_Server<NetcodeSocket>,
    }

    #[pymethods]
    impl Server {
        #[new]
        fn new<'py>(bind_addr: (&str, u16), protocol_id: u64, private_key: Key) -> PyResult<Self> {
            let inner = x_Server::new(bind_addr, protocol_id, private_key).unwrap();
            Ok(Self { inner })
        }

        // TODO: with_config_and_transceiver (can I even?)

        fn update(&mut self, time: f64) -> PyResult<()> {
            self.inner
                .try_update(time)
                .map_err(|e| PyRuntimeError::new_err(e.to_string()))
        }

        fn recv(&mut self) -> PyResult<Option<(Vec<u8>, ClientIndex)>> {
            match self.inner.recv() {
                Some((data, index)) => Ok(Some((data, _netcode::ClientIndex { inner: index }))),
                None => Ok(None),
            }
        }

        fn send(&mut self, data: &[u8], client_idx: &ClientIndex) -> PyResult<()> {
            self.inner
                .send(data, client_idx.inner)
                .map_err(|e| PyRuntimeError::new_err(e.to_string()))
        }

        fn send_all(&mut self, buf: &[u8]) -> PyResult<()> {
            self.inner
                .send_all(buf)
                .map_err(|e| PyRuntimeError::new_err(e.to_string()))
        }

        // TODO: handle optional args

        fn token(&mut self, client_id: u64) -> PyResult<ConnectToken> {
            let token_builder_1 = self.inner.token(client_id);
            let token_1 = token_builder_1.generate().unwrap();
            let token_builder_2 = self.inner.token(client_id);
            let token_2 = token_builder_2.generate().unwrap();
            let bytes = token_2.try_into_bytes().unwrap();
            Ok(ConnectToken {
                inner: token_1,
                bytes,
            })
        }

        fn disconnect(&mut self, client_idx: &ClientIndex) -> PyResult<()> {
            self.inner
                .disconnect(client_idx.inner)
                .map_err(|e| PyRuntimeError::new_err(e.to_string()))
        }

        fn disconnect_all(&mut self) -> PyResult<()> {
            self.inner
                .disconnect_all()
                .map_err(|e| PyRuntimeError::new_err(e.to_string()))
        }

        fn address(&self) -> PyResult<(String, u16)> {
            let addr = self.inner.addr();
            Ok((addr.ip().to_string(), addr.port()))
        }

        fn addr(&self) -> PyResult<(String, u16)> {
            self.address()
        }

        fn num_connected_clients(&self) -> usize {
            self.inner.num_connected_clients()
        }

        fn client_id(&self, client_idx: &ClientIndex) -> Option<u64> {
            self.inner.client_id(client_idx.inner)
        }

        fn client_address(&self, client_idx: &ClientIndex) -> Option<(String, u16)> {
            self.inner
                .client_addr(client_idx.inner)
                .map(|addr| (addr.ip().to_string(), addr.port()))
        }

        fn client_addr(&self, client_idx: &ClientIndex) -> Option<(String, u16)> {
            self.client_address(client_idx)
        }
    }
}
