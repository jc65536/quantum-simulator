use std::fs::File;

use parse::parse_file;
use state::State;

mod parse;
mod prog;
mod state;

fn main() {
    let f = File::open("tests/miller_11.qasm").unwrap();

    let prog = parse_file(f);

    let mut state = State::init(prog.qreg);

    for i in prog.instrs {
        println!("{i:?}");
        state = state.apply(i);

        for w in &state.0 {
            println!("{w}");
        }

        println!("=================");
    }
}
