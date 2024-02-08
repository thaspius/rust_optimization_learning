use pyo3::prelude::*;

fn main() -> PyResult<()> {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let sympy = py.import("sympy")?;

        let expr = sympy.call_method("parse_expr", ("x**2 + x/2 - sin(x)/x",), None)?;
        let diff = sympy.call_method("diff", (expr,), None)?;
        println!("{:?}", diff);
        Ok(())
    })
}
