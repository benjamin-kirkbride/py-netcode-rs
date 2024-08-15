use ::netcode::{
    generate_key as x_generate_key, try_generate_key as x_try_generate_key, Client as x_Client,
    ClientConfig as x_ClientConfig, ClientState as x_ClientState, ConnectToken as x_ConnectToken,
    ConnectTokenBuilder as x_ConnectTokenBuilder, Error as x_Error, InvalidTokenError,
    Key as x_Key, NetcodeSocket, Result as x_Result, Server as x_Server,
    ServerConfig as x_ServerConfig, Transceiver, CONNECT_TOKEN_BYTES as x_CONNECT_TOKEN_BYTES,
    MAX_PACKET_SIZE as x_MAX_PACKET_SIZE, NETCODE_VERSION as x_NETCODE_VERSION,
    PRIVATE_KEY_BYTES as x_PRIVATE_KEY_BYTES, USER_DATA_BYTES as x_USER_DATA_BYTES,
};
use pyo3::prelude::*;
use pyo3::types::*;
use std::net::SocketAddr;
use std::net::ToSocketAddrs;

type Key = [u8; 32];

#[pymodule]
mod netcode {
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
    fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
        Ok((a + b).to_string())
    }

    #[pyfunction]
    fn generate_key(py: Python) -> PyObject {
        // return bytes
        let key = x_generate_key();
        PyBytes::new_bound(py, &key).into()
    }

    /// TODO: try_generate_key?

    #[pyclass]
    struct ConnectToken {
        inner: x_ConnectToken,
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

        // TODO: state

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
}
