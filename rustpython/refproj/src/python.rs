use numpy::PyArray1;
use numpy::PyArrayMethods;
use pyo3::exceptions;
use pyo3::prelude::*;
use std::fmt::Debug;

use crate::rust;

/// Errors from mismatch between python and rust
#[derive(Debug)]
#[allow(dead_code)]
enum PyInteropError {
    DimensionalityError { msg: String },
}

impl From<PyInteropError> for PyErr {
    fn from(val: PyInteropError) -> Self {
        exceptions::PyValueError::new_err(format!("{:#?}", &val))
    }
}

#[pyfunction]
fn hello_from_bin() -> String {
    "Hello from refproj!".to_string()
}

#[pyfunction]
fn nusselt_turbulent_smooth_duct<'py>(
    re: Bound<'py, PyArray1<f64>>,
    pr: Bound<'py, PyArray1<f64>>,
    f: Bound<'py, PyArray1<f64>>,
    out: Bound<'py, PyArray1<f64>>,
) -> PyResult<()> {
    // Calculate
    match rust::nusselt_turbulent_smooth_duct_par(
        re.readonly().as_slice()?,
        pr.readonly().as_slice()?,
        f.readonly().as_slice()?,
        out.readwrite().as_slice_mut()?,
    ) {
        Ok(_) => (),
        Err(x) => {
            let err: PyErr = PyInteropError::DimensionalityError { msg: x.to_string() }.into();
            return Err(err);
        }
    }

    Ok(())
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn refproj(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello_from_bin, m)?)?;
    m.add_function(wrap_pyfunction!(nusselt_turbulent_smooth_duct, m)?)?;
    Ok(())
}
