#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TransactionId([u8; 12]);

impl TransactionId {
    pub const fn new(bytes: [u8; 12]) -> Self {
        Self(bytes)
    }

    pub const fn as_bytes(&self) -> &[u8; 12] {
        &self.0
    }
}
