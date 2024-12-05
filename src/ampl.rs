use std::ops::{Add, AddAssign, Sub};

#[derive(Clone, Copy, PartialEq)]
/// Ampl(a, b) = a + b * sqrt(2)
pub struct Ampl(pub i64, pub i64);

impl Ampl {
    pub fn div_sqrt2(self) -> Self {
        Ampl(self.1, self.0 / 2)
    }

    pub fn to_f64(self) -> f64 {
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

impl Sub for Ampl {
    type Output = Ampl;

    fn sub(self, rhs: Self) -> Self::Output {
        Ampl(self.0 - rhs.0, self.1 - rhs.1)
    }
}
