use pyo3::prelude::*;

#[pymodule]
mod rush_bindings {
    use pyo3::{prelude::*, types::PyList};

    #[pyfunction]
    fn add(x: Bound<'_, PyList>) -> usize {
        for item in x.iter() {
            println!("{:?}", item);
        }
        1
    }
}
