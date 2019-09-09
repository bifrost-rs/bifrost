#[derive(Debug, Eq, PartialEq)]
pub struct Method([u8; 2]);

impl Method {
    pub const BINDING: Self = Self::from_low_12_bits([0, 1]);

    pub const fn from_low_12_bits(mut bits: [u8; 2]) -> Self {
        bits[0] &= 0b1111;
        Self(bits)
    }

    pub const fn as_bytes(&self) -> [u8; 2] {
        self.0
    }
}
