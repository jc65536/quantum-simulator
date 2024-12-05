use std::{env, fs::File};

mod ampl;
mod bitvec;
mod parse;
mod prog;
mod state;

use parse::parse;
use state::State;

fn main() {
    let f = File::open(env::args().skip(1).next().unwrap()).unwrap();

    let prog = parse(f).unwrap();

    let statevec = prog
        .instrs
        .iter()
        .fold(State::init(prog.qreg_used), |state, &ins| state.apply(ins))
        .to_statevec();

    println!("========================================");
    println!("State vector:");
    println!("{statevec:?}")
}
