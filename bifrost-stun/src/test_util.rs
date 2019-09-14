use crate::message::attribute::{Attribute, XorMappedAddress};
use crate::message::{Class, Message, Method, TransactionId};
use bytes::BytesMut;
use std::net::SocketAddr;

pub fn get_test_addrs() -> Vec<SocketAddr> {
    vec![
        "213.141.156.236:48583".parse().unwrap(),
        "[2001:db8:85a3:8d3:1319:8a2e:370:7348]:443"
            .parse()
            .unwrap(),
    ]
}

pub fn new_test_msg(addr: SocketAddr) -> Message {
    let transaction_id = TransactionId::new([3; 12]);
    let attributes = vec![XorMappedAddress(addr).to_raw(&transaction_id)];

    Message {
        class: Class::SuccessResponse,
        method: Method::BINDING,
        transaction_id,
        attributes,
    }
}

pub fn new_reference_msg(addr: SocketAddr) -> BytesMut {
    use bytecodec::EncodeExt;
    use stun_codec::rfc5389::attributes::XorMappedAddress;
    use stun_codec::rfc5389::methods::BINDING;
    use stun_codec::rfc5389::Attribute;
    use stun_codec::{Message, MessageClass, MessageEncoder, TransactionId};

    let mut msg = Message::new(
        MessageClass::SuccessResponse,
        BINDING,
        TransactionId::new([3; 12]),
    );
    msg.add_attribute(Attribute::XorMappedAddress(XorMappedAddress::new(addr)));

    let mut encoder = MessageEncoder::new();
    BytesMut::from(encoder.encode_into_bytes(msg).unwrap())
}
