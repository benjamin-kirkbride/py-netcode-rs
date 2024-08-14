use ::netcode::{
    generate_key as x_generate_key, try_generate_key as x_try_generate_key, Client as x_Client,
    ClientConfig as x_ClientConfig, ClientState as x_ClientState, ConnectToken as x_ConnectToken,
    ConnectTokenBuilder as x_ConnectTokenBuilder, Error as x_Error,
    InvalidTokenError as x_InvalidTokenError, Key as x_Key, Result as x_Result, Server as x_Server,
    ServerConfig as x_ServerConfig, Transceiver as x_Transceiver,
    CONNECT_TOKEN_BYTES as x_CONNECT_TOKEN_BYTES, MAX_PACKET_SIZE as x_MAX_PACKET_SIZE,
    NETCODE_VERSION as x_NETCODE_VERSION, PRIVATE_KEY_BYTES as x_PRIVATE_KEY_BYTES,
    USER_DATA_BYTES as x_USER_DATA_BYTES,
};
use pyo3::prelude::*;
use pyo3::types::*;

#[pymodule]
mod netcode {

    use super::*;

    #[pyfunction]
    fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
        Ok((a + b).to_string())
    }

    #[pyfunction]
    fn generate_key() -> PyResult<x_Key> {
        Ok(x_generate_key())
    }
}
