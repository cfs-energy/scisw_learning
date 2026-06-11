//! Example calculations.
//! As much as possible, these should work with primitives, slices of primitives,
//! and iterators over primitives so that they don't force too many opinions
//! on upstream operations.
use core::num::NonZeroUsize;

use rayon::{
    iter::{IntoParallelIterator, ParallelIterator},
    slice::{ParallelSlice, ParallelSliceMut},
};
use serde::{Deserialize, Serialize};

#[cfg(feature = "python")]
use pyo3::prelude::*;

/// Chunk size for parallelism
#[inline]
pub(crate) fn chunksize(nelem: usize) -> usize {
    let ncores = std::thread::available_parallelism()
        .unwrap_or(NonZeroUsize::MIN)
        .get();

    (nelem / ncores).max(1)
}

/// Gnielinski correlation for heat transfer in turbulent pipes.
///
/// Valid region:
///     Re: in [3e3, 5e6]
///     Pr: in [0.5, inf]
///     f: anywhere the source correlation is valid
///
/// Note that this will produce unhelpful values below its valid range of Re,
/// and should be blended into the constant laminar Nusselt number,
/// which is typically given as Nu_laminar = 3.66.
#[inline]
pub fn nusselt_turbulent_smooth_duct(re: f64, pr: f64, f: f64) -> f64 {
    let num = (f / 8.0) * (re - 1000.0) * pr;
    let denom = 1.0 + 12.7 * (f / 8.0).sqrt() * ((pr * pr).cbrt() - 1.0);
    let nu = num / denom;
    return nu; // [dimensionless] Nusselt number
}

/// Gnielinski correlation for heat transfer in turbulent pipes.
/// See [nusselt_turbulent_smooth_duct] for more info.
#[inline]
pub fn nusselt_turbulent_smooth_duct_vec(
    re: &[f64],
    pr: &[f64],
    f: &[f64],
    out: &mut [f64],
) -> Result<(), &'static str> {
    let m = out.len();
    if re.len() != m || pr.len() != m || f.len() != m {
        return Err("Length mismatch");
    }

    // Run scalar loop
    for i in 0..m {
        out[i] = nusselt_turbulent_smooth_duct(re[i], pr[i], f[i]);
    }

    Ok(())
}

/// Gnielinski correlation for heat transfer in turbulent pipes.
/// See [nusselt_turbulent_smooth_duct] for more info.
#[inline]
pub fn nusselt_turbulent_smooth_duct_par(
    re: &[f64],
    pr: &[f64],
    f: &[f64],
    out: &mut [f64],
) -> Result<(), &'static str> {
    // Choose size of chunks,
    // using a hand-tuned heuristic to decide when to switch
    // from serial to threaded.
    let n = if re.len() > 10_000 {
        chunksize(re.len())
    } else {
        re.len()
    };
    // let n = re.len();  // uncomment to test singlethreaded

    // Chunk inputs, shadowing original names
    let rec = re.par_chunks(n);
    let prc = pr.par_chunks(n);
    let fc = f.par_chunks(n);
    let outc = out.par_chunks_mut(n);

    // Iterate over each chunk in parallel, shadowing original names again
    (outc, rec, prc, fc)
        .into_par_iter()
        .try_for_each(|(out, re, pr, f)| {
            // Run scalar loop
            nusselt_turbulent_smooth_duct_vec(re, pr, f, out)?;

            Ok(())
        })?;

    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(
    feature = "python",
    pyclass(module = "refproj.refproj", skip_from_py_object)
)]
pub struct Cat {
    pub name: String,
}

impl Cat {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }

    pub fn from_json(json: &str) -> serde_json::Result<Self> {
        serde_json::from_str(json)
    }
}

#[cfg(feature = "python")]
fn json_err_to_py(err: serde_json::Error) -> PyErr {
    pyo3::exceptions::PyValueError::new_err(err.to_string())
}

#[cfg(feature = "python")]
#[pymethods]
impl Cat {
    #[new]
    fn py_new(name: String) -> Self {
        Cat::new(name)
    }

    #[getter]
    fn name(&self) -> &str {
        &self.name
    }

    #[setter]
    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    #[pyo3(name = "to_json")]
    fn py_to_json(&self) -> PyResult<String> {
        self.to_json().map_err(json_err_to_py)
    }

    #[staticmethod]
    #[pyo3(name = "from_json")]
    fn py_from_json(json: &str) -> PyResult<Self> {
        Cat::from_json(json).map_err(json_err_to_py)
    }
}

#[cfg_attr(feature = "python", pyfunction)]
pub fn pet(cat: &Cat) -> String {
    format!("You pet {}.", cat.name)
}

#[cfg(test)]
mod tests {
    use super::{Cat, pet};

    #[test]
    fn cat_json_round_trip_and_pet() {
        let cat = Cat::new("Mochi".to_string());

        assert_eq!(cat.to_json().unwrap(), r#"{"name":"Mochi"}"#);
        assert_eq!(Cat::from_json(&cat.to_json().unwrap()).unwrap(), cat);
        assert_eq!(pet(&cat), "You pet Mochi.");
    }
}
