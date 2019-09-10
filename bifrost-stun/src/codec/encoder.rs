use bytes::{BufMut, BytesMut};
use std::io;
use tokio_codec::Encoder;

use crate::{
    codec::{MessageCodec, ATTR_HEADER_LEN, HEADER_LEN},
    message::{Class, Message, Method, RawAttribute, TransactionId, MAGIC_COOKIE},
};

impl Encoder for MessageCodec {
    type Item = Message;
    type Error = io::Error;

    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let attrs_len: usize = item
            .attributes
            .iter()
            .map(|a| ATTR_HEADER_LEN as usize + a.padded_len() as usize)
            .sum();

        // TODO: Make maximum length customizable.
        if attrs_len >= (u16::max_value() - HEADER_LEN) as usize {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "attributes length too large",
            ));
        }
        let attrs_len = attrs_len as u16;
        let total_len = HEADER_LEN + attrs_len;

        dst.reserve(total_len as usize);
        encode_class_and_method(item.class, item.method, dst);
        encode_len(attrs_len, dst);
        encode_magic_cookie(dst);
        encode_transaction_id(&item.transaction_id, dst);
        encode_attributes(&item.attributes, dst);

        Ok(())
    }
}

fn encode_class_and_method(class: Class, method: Method, dst: &mut BytesMut) {
    let c = class.as_byte();
    let m = method.as_bytes();

    dst.put_slice(&[
        (m[0] << 2) | (m[1] >> 7 << 1) | (c >> 1),
        ((m[1] & 0b111_0000) << 1) | ((c & 0b1) << 4) | (m[1] & 0b1111),
    ]);
}

fn encode_len(len: u16, dst: &mut BytesMut) {
    assert_eq!(len % 4, 0);
    dst.put_u16_be(len);
}

fn encode_magic_cookie(dst: &mut BytesMut) {
    dst.put_slice(&MAGIC_COOKIE);
}

fn encode_transaction_id(tr_id: &TransactionId, dst: &mut BytesMut) {
    dst.put_slice(tr_id.as_bytes());
}

fn encode_attributes(attrs: &[RawAttribute], dst: &mut BytesMut) {
    for attr in attrs {
        dst.put_u16_be(attr.r#type());
        dst.put_u16_be(attr.unpadded_len());
        dst.put_slice(attr.value());
        for _ in attr.unpadded_len()..attr.padded_len() {
            dst.put_u8(0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util;

    #[test]
    fn success() {
        for addr in test_util::get_test_addrs() {
            let msg = test_util::new_test_msg(addr);
            let mut bytes = BytesMut::new();
            let mut codec = MessageCodec::new();
            codec.encode(msg, &mut bytes).unwrap();

            let expected = test_util::new_reference_msg(addr);
            assert_eq!(bytes, expected);
        }
    }
}
