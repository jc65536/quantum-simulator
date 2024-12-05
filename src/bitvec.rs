#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct BitVec(pub u64);

impl BitVec {
    pub fn get_bit(&self, i: u32) -> u32 {
        (self.0 >> i) as u32 & 1
    }

    pub fn set_bit(&self, i: u32, b: u32) -> Self {
        Self(self.0 & !(1 << i) | (b as u64) << i)
    }

    pub fn flip_bit(&self, i: u32) -> Self {
        Self(self.0 ^ 1 << i)
    }

    pub fn to_be_string(&self, n: u32) -> String {
        (0..n).rev().map(|i| self.get_bit(i).to_string()).collect()
    }

    pub fn to_be_u64(&self, n: u32) -> u64 {
        (0..n)
            .map(|i| self.get_bit(i))
            .fold(0, |acc, b| (acc << 1) + b as u64)
    }
}
