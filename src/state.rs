use core::f64;
use std::{
    collections::HashMap,
    fmt::Display,
    hash::Hash,
    ops::{Add, AddAssign, Neg, Sub},
};

use crate::prog::Ins;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct BitVec(u32);

impl BitVec {
    fn get_bit(&self, i: u32) -> u32 {
        self.0 >> i & 1
    }

    fn set_bit(&self, i: u32, b: u32) -> Self {
        Self(self.0 & !(1 << i) | b << i)
    }

    fn flip_bit(&self, i: u32) -> Self {
        Self(self.0 ^ 1 << i)
    }

    pub fn to_vec(&self, n: u32) -> Vec<u32> {
        (0..n).map(|i| self.get_bit(i)).collect()
    }
}

#[derive(Clone, Copy, PartialEq)]
/// Ampl(a, b) = a + b * sqrt(2)
struct Ampl(i32, i32);

impl Ampl {
    fn div_sqrt2(self) -> Self {
        Ampl(self.1, self.0 / 2)
    }

    fn is_zero(self) -> bool {
        self.0 == 0 && self.1 == 0
    }

    fn to_float(self) -> f64 {
        self.0 as f64 + self.1 as f64 * 2f64.sqrt()
    }
}

impl Add for Ampl {
    type Output = Ampl;

    fn add(self, rhs: Self) -> Self::Output {
        Ampl(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for Ampl {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Neg for Ampl {
    type Output = Ampl;

    fn neg(self) -> Self::Output {
        Ampl(-self.0, -self.1)
    }
}

impl Sub for Ampl {
    type Output = Ampl;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl PartialOrd for Ampl {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.to_float().partial_cmp(&other.to_float())
    }
}

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
                    ampl: -self.ampl.div_sqrt2(),
                    phase: self.phase,
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
                phase: (self.phase + self.vec.get_bit(i) * 7) % 8,
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
}

impl Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Ampl: {:.3}, Phase: {} * pi / 4, Vec: |{}>",
            self.ampl.to_float(),
            self.phase,
            self.vec.to_vec(16).into_iter().map(|b| b.to_string()).collect::<Vec<_>>().join(""),
        )
    }
}

pub struct State(pub Vec<World>);

impl State {
    pub fn init(n: u32) -> Self {
        Self(vec![World::init(n)])
    }

    pub fn apply(self, ins: Ins) -> Self {
        let mut compact_map: HashMap<BitVec, [Ampl; 8]> = HashMap::new();

        for w_new in self.0.into_iter().flat_map(|w| w.apply(ins)) {
            compact_map
                .entry(w_new.vec)
                .and_modify(|arr| arr[w_new.phase as usize] += w_new.ampl)
                .or_insert_with(|| {
                    let mut arr = [Ampl(0, 0); 8];
                    arr[w_new.phase as usize] = w_new.ampl;
                    arr
                });
        }

        let worlds: Vec<World> = compact_map
            .into_iter()
            .flat_map(|(vec, arr)| {
                (0..4).filter_map(move |i| {
                    let a1 = arr[i];
                    let a2 = arr[(i + 4) % 8];
                    if a1 == a2 {
                        None
                    } else if a1 > a2 {
                        Some(World {
                            ampl: a1 - a2,
                            phase: i as u32,
                            vec,
                        })
                    } else {
                        Some(World {
                            ampl: a2 - a1,
                            phase: (i as u32 + 4) % 8,
                            vec,
                        })
                    }
                })
            })
            .collect();

        Self(worlds)
    }
}
