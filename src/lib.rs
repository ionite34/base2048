#[macro_use]
extern crate lazy_static;

use pyo3::create_exception;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

mod encoder;

create_exception!(base2048, DecodeError, PyValueError);

/// Encode bytes using base2048
#[pyfunction]
fn encode(s: &[u8]) -> String {
    encoder::encode(s)
}

/// Decode a base2048 encoded string into bytes
#[pyfunction]
fn decode(s: &str) -> PyResult<PyObject> {
    match encoder::decode(s) {
        Ok(v) => Ok(Python::with_gil(|py| PyBytes::new(py, &v).into())),
        Err(e) => Err(DecodeError::new_err(e)),
    }
}

#[pymodule]
#[pyo3(name = "base2048")]
fn module(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(encode, m)?)?;
    m.add_function(wrap_pyfunction!(decode, m)?)?;
    m.add("DecodeError", _py.get_type::<DecodeError>())?;
    Ok(())
}
