use core::f64;
use std::{collections::HashMap, env, fmt::Display};

use num_complex::Complex64;

use crate::{ampl::Ampl, bitvec::BitVec, prog::Ins};

pub struct NbitWorld<'a>(u32, &'a World);

pub struct World {
    /// Counts fractions of 2 ** (-n / 2) where n is number of qubits
    pub ampl: Ampl,
    /// Counts in multiples of pi / 4
    phase: u32,
    pub vec: BitVec,
}

impl World {
    pub fn init(n: u32) -> Self {
        Self {
            ampl: if n % 2 == 0 {
                Ampl(1 << n / 2, 0)
            } else {
                Ampl(0, 1 << n / 2)
            },
            phase: 0,
            vec: BitVec(0),
        }
    }

    fn apply(self, ins: Ins) -> Vec<Self> {
        match ins {
            Ins::H(i) => vec![
                Self {
                    ampl: self.ampl.div_sqrt2(),
                    phase: self.phase,
                    vec: self.vec.set_bit(i, 0),
                },
                Self {
                    ampl: self.ampl.div_sqrt2(),
                    phase: (self.phase + 4 * self.vec.get_bit(i)) % 8,
                    vec: self.vec.set_bit(i, 1),
                },
            ],

            Ins::X(i) => vec![Self {
                vec: self.vec.flip_bit(i),
                ..self
            }],

            Ins::T(i) => vec![Self {
                phase: (self.phase + self.vec.get_bit(i)) % 8,
                ..self
            }],

            Ins::Tdg(i) => vec![Self {
                phase: (self.phase + 7 * self.vec.get_bit(i)) % 8,
                ..self
            }],

            Ins::Cx(c, t) => vec![Self {
                vec: if self.vec.get_bit(c) == 1 {
                    self.vec.flip_bit(t)
                } else {
                    self.vec
                },
                ..self
            }],
        }
    }

    pub fn bits(&self, n: u32) -> NbitWorld {
        NbitWorld(n, self)
    }
}

impl Display for NbitWorld<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let &Self(n, w) = self;
        let ampl = w.ampl.to_f64() / 2f64.powf(n as f64 / 2.0);
        let phase = w.phase as f64 * f64::consts::PI / 4.0;
        let x: f64 = ampl * f64::cos(phase);
        let y: f64 = ampl * f64::sin(phase);
        write!(
            f,
            "Ampl: {ampl:.3} * exp(i * {}pi / 4) = {x:.3} + {y:.3}i, Vec: |{}> = |{}>",
            w.phase,
            w.vec.to_be_string(n),
            w.vec.to_be_u64(n),
        )
    }
}

pub struct State {
    pub n: u32,
    pub worlds: Vec<World>,
}

impl State {
    pub fn init(n: u32) -> Self {
        Self {
            n,
            worlds: vec![World::init(n)],
        }
    }

    pub fn apply(self, ins: Ins) -> Self {
        let print_steps = env::var("PRINT_STEPS").is_ok_and(|s| !s.is_empty());

        if print_steps {
            println!("========================================");
            println!("Instruction: {:?}", ins);
        }

        let mut compact_map: HashMap<BitVec, [Ampl; 8]> = HashMap::new();

        // Collect worlds by bit vector
        for w_new in self.worlds.into_iter().flat_map(|w| w.apply(ins)) {
            compact_map
                .entry(w_new.vec)
                .and_modify(|arr| arr[w_new.phase as usize] += w_new.ampl)
                .or_insert_with(|| {
                    let mut arr = [Ampl(0, 0); 8];
                    arr[w_new.phase as usize] = w_new.ampl;
                    arr
                });
        }

        // Combine worlds with same bit vector and compatible phase
        let worlds: Vec<World> = compact_map
            .into_iter()
            .flat_map(|(vec, arr)| {
                (0..4).filter_map(move |i| {
                    let a1 = arr[i];
                    let a2 = arr[(i + 4) % 8];
                    if a1 == a2 {
                        None
                    } else {
                        Some(World {
                            ampl: a1 - a2,
                            phase: i as u32,
                            vec,
                        })
                    }
                })
            })
            .collect();

        if print_steps {
            for w in &worlds {
                println!("{}", w.bits(self.n));
            }
        }

        Self { worlds, ..self }
    }

    pub fn to_statevec(self) -> Vec<Complex64> {
        let mut ret = vec![Complex64::ZERO; 2usize.pow(self.n)];
        for w in self.worlds {
            ret[w.vec.to_be_u64(self.n) as usize] = Complex64::from_polar(
                w.ampl.to_f64() / 2f64.powf(self.n as f64 / 2.0),
                (w.phase as f64) * f64::consts::PI / 4.0,
            );
        }
        ret
    }
}
