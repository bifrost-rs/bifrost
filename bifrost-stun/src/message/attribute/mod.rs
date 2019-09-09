mod xor_mapped_address;

pub use self::xor_mapped_address::XorMappedAddress;

use crate::message::{RawAttribute, TransactionId};

pub trait Attribute: Sized {
    const TYPE: u16;

    fn from_raw(raw: &[u8], tr_id: &TransactionId) -> Option<Self>;

    fn to_raw(&self, tr_id: &TransactionId) -> RawAttribute;
}
