use pyo3::prelude::*;

/// A simple function that adds two numbers
#[pyfunction]
fn add(a: i64, b: i64) -> i64 {
    a + b
}

/// A Rust struct that Python can use
#[pyclass]
struct Counter {
    value: i64,
}

#[pymethods]
impl Counter {
    #[new]
    fn new(initial: i64) -> Self {
        Counter { value: initial }
    }

    fn increment(&mut self) {
        self.value += 1;
    }

    fn get_value(&self) -> i64 {
        self.value
    }
}

/// Module definition - this is what Python imports
#[pymodule]
fn a_rust_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add, m)?)?;
    m.add_class::<Counter>()?;
    Ok(())
}
