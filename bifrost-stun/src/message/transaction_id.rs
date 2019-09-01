#[derive(Debug, Eq, PartialEq)]
pub struct TransactionId([u8; 12]);

impl TransactionId {
    pub const fn new(bytes: [u8; 12]) -> Self {
        Self(bytes)
    }

    pub const fn bytes(&self) -> &[u8; 12] {
        &self.0
    }
}
