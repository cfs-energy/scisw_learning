use serde::{Deserialize, Serialize};

#[cfg(feature = "python")]
use pyo3::prelude::*;

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
