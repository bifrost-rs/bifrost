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
        let len: usize = item
            .attributes
            .iter()
            .map(|a| usize::from(ATTR_HEADER_LEN + a.padded_len()))
            .sum();

        if len >= (u16::max_value() - HEADER_LEN) as usize {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "attributes length too large",
            ));
        }
        let len = len as u16;

        dst.reserve((HEADER_LEN + len) as usize);
        encode_class_and_method(item.class, item.method, dst);
        encode_len(len, dst);
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
        dst.put_u16_be(attr.r#type);
        dst.put_u16_be(attr.unpadded_len());
        dst.put_slice(attr.value());
        for _ in attr.unpadded_len()..attr.padded_len() {
            dst.put_u8(0);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::net::SocketAddr;

    use super::*;
    use crate::message::attribute::{Attribute, XorMappedAddress};

    fn get_test_addrs() -> Vec<SocketAddr> {
        vec![
            "213.141.156.236:48583".parse().unwrap(),
            "[2001:db8:85a3:8d3:1319:8a2e:370:7348]:443"
                .parse()
                .unwrap(),
        ]
    }

    fn new_test_msg(addr: SocketAddr) -> Message {
        let transaction_id = TransactionId::new([3; 12]);
        let attributes = vec![XorMappedAddress(addr).to_raw(&transaction_id)];

        Message {
            class: Class::SuccessResponse,
            method: Method::BINDING,
            transaction_id,
            attributes,
        }
    }

    fn new_test_msg_bytes(addr: SocketAddr) -> BytesMut {
        use bytecodec::EncodeExt;
        use stun_codec::{
            rfc5389::{attributes::XorMappedAddress, methods::BINDING, Attribute},
            Message, MessageClass, MessageEncoder, TransactionId,
        };

        let mut msg = Message::new(
            MessageClass::SuccessResponse,
            BINDING,
            TransactionId::new([3; 12]),
        );
        msg.add_attribute(Attribute::XorMappedAddress(XorMappedAddress::new(addr)));

        let mut encoder = MessageEncoder::new();
        BytesMut::from(encoder.encode_into_bytes(msg).unwrap())
    }

    #[test]
    fn test_success() {
        for addr in get_test_addrs() {
            let mut codec = MessageCodec::new();
            let msg = new_test_msg(addr);
            let mut bytes = BytesMut::new();
            codec.encode(msg, &mut bytes).unwrap();

            let expected = new_test_msg_bytes(addr);
            assert_eq!(bytes.len(), expected.len());
            assert_eq!(bytes, expected);
        }
    }
}
