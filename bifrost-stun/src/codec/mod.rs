mod decoder;
mod encoder;

use crate::message::{Class, Method, TransactionId};

const HEADER_LEN: u16 = 20;
const ATTR_HEADER_LEN: u16 = 4;

#[derive(Default)]
pub struct MessageCodec {
    header: Option<(Class, Method, u16, TransactionId)>,
}

impl MessageCodec {
    pub fn new() -> Self {
        Self::default()
    }
}
