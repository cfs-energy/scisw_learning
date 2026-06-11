use numpy::borrow::{PyReadonlyArray1, PyReadwriteArray1};
use pyo3::exceptions;
use pyo3::prelude::*;
use std::collections::HashMap;
use std::fmt::Debug;

use crate::{cat, rust};

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
fn nusselt_turbulent_smooth_duct(
    re: PyReadonlyArray1<f64>,
    pr: PyReadonlyArray1<f64>,
    f: PyReadonlyArray1<f64>,
    mut out: PyReadwriteArray1<f64>,
) -> PyResult<()> {
    // Calculate
    match rust::nusselt_turbulent_smooth_duct_par(
        re.as_slice()?,
        pr.as_slice()?,
        f.as_slice()?,
        out.as_slice_mut()?,
    ) {
        Ok(_) => (),
        Err(x) => {
            let err: PyErr = PyInteropError::DimensionalityError { msg: x.to_string() }.into();
            return Err(err);
        }
    }

    Ok(())
}

/// A hodgepodge of things that can be mapped between Rust and Python automatically.
///
/// This interface is simpler than using borrowed numpy arrays, but is less performant,
/// because the mapped inputs like Vec and HashMap are copied from python instead of being
/// passed by reference.
///
/// The rest of the signature still has to be defined in the type stubs, but we can set
/// defaults here. Type hints like the following do not work here and must be done in the stubs.
/// #[pyfunction(signature = (a: list[float], b: dict[str, str], c: str, n: int | None = None))]
/// ^ (that doesn't work)
#[pyfunction(signature = (a, b, c, n=None))]
fn jumble(
    a: Vec<f64>,
    b: HashMap<String, String>,
    c: &str,
    n: Option<i64>,
) -> Option<(String, Vec<f64>, HashMap<String, Option<i64>>)> {
    let spam = format!(
        "{}, {}, {}, {}",
        a.iter().sum::<f64>(),
        b.len(),
        c,
        n.unwrap_or(0)
    );
    let numbers = a.iter().map(|x| 1.1 * x).collect();
    let mut config_maybe = HashMap::new();
    config_maybe.insert(c.to_string(), n);
    Some((spam, numbers, config_maybe))
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn refproj(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<cat::Cat>()?;
    m.add_function(wrap_pyfunction!(hello_from_bin, m)?)?;
    m.add_function(wrap_pyfunction!(cat::pet, m)?)?;
    m.add_function(wrap_pyfunction!(nusselt_turbulent_smooth_duct, m)?)?;
    m.add_function(wrap_pyfunction!(jumble, m)?)?;
    Ok(())
}
