pub mod attribute;

mod class;
mod method;
mod raw_attribute;
mod transaction_id;

pub use self::{
    class::Class, method::Method, raw_attribute::RawAttribute, transaction_id::TransactionId,
};

use crate::message::attribute::Attribute;

pub(crate) const MAGIC_COOKIE: [u8; 4] = [0x21, 0x12, 0xa4, 0x42];

/// Represents a STUN message, defined in
/// [RFC 5389](https://tools.ietf.org/html/rfc5389#section-6).
#[derive(Debug)]
pub struct Message {
    pub class: Class,
    pub method: Method,
    pub transaction_id: TransactionId,
    pub attributes: Vec<RawAttribute>,
}

impl Message {
    pub fn attr<T: Attribute>(&self) -> Option<T> {
        self.attributes.iter().find_map(|attr| {
            if attr.r#type() == T::TYPE {
                T::from_raw(attr.value(), &self.transaction_id)
            } else {
                None
            }
        })
    }
}
