#[derive(Debug, Clone, Copy)]
pub enum Ins {
    H(u32),
    X(u32),
    T(u32),
    Tdg(u32),
    Cx(u32, u32),
}

#[derive(Debug)]
pub struct Prog {
    pub qreg: u32,
    pub qreg_used: u32,
    pub creg: u32,
    pub instrs: Vec<Ins>,
}
