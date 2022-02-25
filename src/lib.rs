use pyo3::prelude::*;
use kathoey;

#[pyclass(name = "Kathoey")]
pub struct Kathoey {
  k: kathoey::types::Kathoey
}

#[pymethods]
impl Kathoey {
  #[new]
  fn __new__(s: &str) -> Self {
    Kathoey { k: kathoey::types::Kathoey::load(s).unwrap() }
  }

  fn kathoey(&self, s: &str) -> String {
    self.k.feminize(s)
  }
}

#[pymodule]
pub fn decorator(_py: Python, module: &PyModule) -> PyResult<()> {
  module.add_class::<Kathoey>()?;
  Ok(())
}
