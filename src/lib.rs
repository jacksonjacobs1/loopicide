use pyo3::prelude::*;
mod loops;
use loops::sum_as_string;

/// Formats the sum of two numbers as string.


/// A Python module implemented in Rust.
#[pymodule]
fn loopicide(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
