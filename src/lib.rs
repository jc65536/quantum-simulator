use pyo3::prelude::*;
mod ampl;
mod bitvec;
mod parse;
mod prog;
mod state;

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
mod cs238 {
    use num_complex::Complex64;
    use pyo3::prelude::*;

    use crate::parse::parse;
    use crate::state::State;

    #[pyfunction]
    fn simulate(s: String) -> PyResult<Vec<Complex64>> {
        let prog = parse(s.as_bytes()).expect("parse error");

        let statevec = prog
            .instrs
            .iter()
            .fold(State::init(prog.qreg_used), |state, &ins| state.apply(ins))
            .to_statevec();

        Ok(statevec)
    }
}
