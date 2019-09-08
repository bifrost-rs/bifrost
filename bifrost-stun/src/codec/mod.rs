mod decoder;

use crate::message::{Class, Method, TransactionId};

#[derive(Default)]
pub struct MessageCodec {
    header: Option<(Class, Method, u16, TransactionId)>,
}

impl MessageCodec {
    pub fn new() -> Self {
        Self::default()
    }

    fn reset(&mut self) {
        self.header = None;
    }
}
