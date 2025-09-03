use pyo3::prelude::*;
use wifi_config::send_wifi_to_network_manager;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn wifi_config_send(ssid: String, password: String) -> PyResult<()> {
    send_wifi_to_network_manager(&ssid, &password);
    Ok(())
}

/// A Python module implemented in Rust.
#[pymodule]
fn simple_wifi_config(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(wifi_config_send, m)?)?;
    Ok(())
}
