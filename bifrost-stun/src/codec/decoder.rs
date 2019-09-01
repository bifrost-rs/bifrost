use bytes::BytesMut;
use nom::{
    bits::bits, bytes::complete::take, combinator::verify, multi::many0, number::complete::be_u16,
    IResult,
};
use std::{convert::TryInto, io};
use tokio_codec::Decoder;

use crate::{
    codec::StunCodec,
    message::{Class, Message, Method, RawAttribute, TransactionId, MAGIC_COOKIE},
};

impl Decoder for StunCodec {
    type Item = Option<Message>;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        // TODO: Optimize by parsing the header at most once.
        let (class, method, transaction_id, len) = match parse_header(src) {
            Ok((_, header)) => header,
            Err(nom::Err::Incomplete(_)) => return Ok(None),
            Err(_) => return Ok(Some(None)),
        };

        if src.len() < (20 + len).into() {
            return Ok(None);
        }

        let attr_buf = &src[20..20 + usize::from(len)];
        let attributes = match many0(parse_attribute)(attr_buf) {
            Ok((rest, _)) if !rest.is_empty() => return Ok(Some(None)),
            Ok((_, attrs)) => attrs,
            Err(_) => return Ok(Some(None)),
        };

        src.advance(20 + usize::from(len));

        Ok(Some(Some(Message {
            class,
            method,
            transaction_id,
            attributes,
        })))
    }
}

fn parse_header(input: &[u8]) -> IResult<&[u8], (Class, Method, TransactionId, u16)> {
    use nom::{
        bytes::streaming::{tag, take},
        number::streaming::be_u16,
    };

    let (rest, (class, method)) = bits(parse_class_and_method)(input)?;

    // The message length MUST contain the size, in bytes, of the message not
    // including the 20-byte STUN header. Since all STUN attributes are padded
    // to a multiple of 4 bytes, the last 2 bits of this field are always zero.
    let (rest, len) = verify(be_u16, |x| x % 4 == 0)(rest)?;

    // The magic cookie field MUST contain the fixed value 0x2112A442 in network
    // byte order.
    let (rest, _) = tag(MAGIC_COOKIE)(rest)?;

    // The transaction ID is a 96-bit identifier, used to uniquely identify STUN
    // transactions.
    let (rest, tr_id) = take(12usize)(rest)?;
    let tr_id = TransactionId::new(tr_id.try_into().unwrap());

    Ok((rest, (class, method, tr_id, len)))
}

fn parse_class_and_method(input: (&[u8], usize)) -> IResult<(&[u8], usize), (Class, Method)> {
    use nom::bits::streaming::{tag, take};

    // The most significant 2 bits of every STUN message MUST be zeroes.
    let (rest, _) = tag(0u8, 2usize)(input)?;

    let (rest, m1): (_, u8) = take(5usize)(rest)?;
    let (rest, c1): (_, u8) = take(1usize)(rest)?;
    let (rest, m2): (_, u8) = take(3usize)(rest)?;
    let (rest, c2): (_, u8) = take(1usize)(rest)?;
    let (rest, m3): (_, u8) = take(4usize)(rest)?;

    let class = Class::from_low_2_bits(c1 << 1 | c2);
    let method = Method::from_low_12_bits([m1 >> 1, m1 << 3 | m2 << 4 | m3]);

    Ok((rest, (class, method)))
}

fn parse_attribute(input: &[u8]) -> IResult<&[u8], RawAttribute> {
    // 16-bit type.
    let (rest, r#type) = be_u16(input)?;

    // The value in the 16-bit length field MUST contain the length of the Value
    // part of the attribute, prior to padding, measured in bytes.
    let (rest, len) = verify(be_u16, |x| *x <= RawAttribute::MAX_LEN)(rest)?;

    // Since STUN aligns attributes on 32-bit boundaries, attributes whose
    // content is not a multiple of 4 bytes are padded with 1, 2, or 3 bytes of
    // padding so that its value contains a multiple of 4 bytes. The padding
    // bits are ignored, and may be any value.
    let padded_len = (len + 3) & !0b11;
    let (rest, value) = take(padded_len)(rest)?;
    let value = Vec::from(&value[..len.into()]);

    Ok((rest, RawAttribute { r#type, value }))
}

#[cfg(test)]
mod tests {
    use std::net::SocketAddr;

    use super::*;
    use crate::message::attribute::XorMappedAddress;

    fn new_test_msg(addr: SocketAddr) -> BytesMut {
        use bytecodec::EncodeExt;
        use stun_codec::{
            rfc5389::{attributes::XorMappedAddress, methods::BINDING, Attribute},
            Message, MessageClass, MessageEncoder, TransactionId,
        };

        let mut message = Message::new(
            MessageClass::SuccessResponse,
            BINDING,
            TransactionId::new([3; 12]),
        );
        message.add_attribute(Attribute::XorMappedAddress(XorMappedAddress::new(addr)));

        let mut encoder = MessageEncoder::new();
        let bytes = encoder.encode_into_bytes(message.clone()).unwrap();

        BytesMut::from(bytes)
    }

    #[test]
    fn test_success() {
        let addr = "213.141.156.236:48583".parse().unwrap();
        let mut bytes = new_test_msg(addr);
        bytes.extend(0..3);

        let mut codec = StunCodec;
        let msg = match codec.decode(&mut bytes) {
            Ok(Some(Some(msg))) => msg,
            _ => panic!("failed to decode"),
        };

        assert_eq!(msg.class, Class::SuccessResponse);
        assert_eq!(msg.method, Method::BINDING);
        assert_eq!(msg.transaction_id, TransactionId::new([3; 12]));
        assert_eq!(
            msg.attr::<XorMappedAddress>(),
            Some(XorMappedAddress { addr })
        );
        assert_eq!(bytes.len(), 3);
    }

    #[test]
    fn test_incomplete_header() {
        let addr = "213.141.156.236:48583".parse().unwrap();
        let mut bytes = new_test_msg(addr);
        assert!(bytes.len() > 10);
        unsafe {
            bytes.set_len(10);
        }

        let mut codec = StunCodec;
        match codec.decode(&mut bytes) {
            Ok(None) => (),
            _ => panic!("failed to decode incomplete header"),
        };
        assert_eq!(bytes.len(), 10);
    }

    #[test]
    fn test_incomplete_attrs() {
        let addr = "213.141.156.236:48583".parse().unwrap();
        let mut bytes = new_test_msg(addr);
        bytes.truncate(3);
        let len = bytes.len();

        let mut codec = StunCodec;
        match codec.decode(&mut bytes) {
            Ok(None) => (),
            _ => panic!("failed to decode incomplete attributes"),
        };
        assert_eq!(bytes.len(), len);
    }

    #[test]
    fn test_failure() {
        let mut bytes = BytesMut::from(&b"nonsense"[..]);
        let len = bytes.len();

        let mut codec = StunCodec;
        match codec.decode(&mut bytes) {
            Ok(Some(None)) => (),
            x => panic!("failed to decode non-STUN message {:?}", x),
        };
        assert_eq!(bytes.len(), len);
    }
}
