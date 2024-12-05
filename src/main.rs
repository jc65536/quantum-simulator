use std::{env, fs::File, time::Instant};

mod ampl;
mod bitvec;
mod parse;
mod prog;
mod state;

use parse::parse;
use state::State;

fn test_one_file() {
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

fn test_qubits() {
    let mut prog_string: String = String::from(
        "
OPENQASM 2.0;
include \"qelib1.inc\";
qreg q[24];
creg c[24];
h q[0];
h q[1];
",
    );

    let mut is: Vec<u32> = Vec::new();
    let mut times: Vec<u32> = Vec::new();

    for i in 2..24 {
        prog_string = prog_string + &format!("h q[{i}];");

        let now = Instant::now();

        let prog = parse(prog_string.as_bytes()).unwrap();

        let _ = prog
            .instrs
            .iter()
            .fold(State::init(prog.qreg_used), |state, &ins| state.apply(ins))
            .to_statevec();
    
        let elapsed = now.elapsed();

        println!("========================================");
        println!("q = {i}");
        // println!("{statevec:?}");
        println!("Time elapsed: {} ms", elapsed.as_millis());
        is.push(i);
        times.push(elapsed.as_millis() as u32);
    }

    println!("{is:?}");
    println!("{times:?}");
}

fn main() {
    test_qubits();
}
