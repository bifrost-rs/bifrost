use bytes::BytesMut;
use nom::{
    bits::bits, bytes::complete::take, combinator::verify, multi::many0, number::complete::be_u16,
    IResult,
};
use std::{convert::TryInto, io};
use tokio_codec::Decoder;

use crate::{
    codec::{MessageCodec, HEADER_LEN},
    message::{Class, Message, Method, RawAttribute, TransactionId, MAGIC_COOKIE},
};

impl Decoder for MessageCodec {
    type Item = Option<Message>;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        // The header is parsed only once in case a message isn't fully
        // available at once.
        if self.header.is_none() {
            // TODO: Make maximum length customizable.
            let max_len = u16::max_value() - HEADER_LEN;
            self.header = match parse_header_streaming(src, max_len) {
                Ok((_, header)) => Some(header),
                Err(nom::Err::Incomplete(_)) => return Ok(None),
                Err(_) => return Ok(Some(None)),
            };
        }
        // `self.header` is guaranteed to be a `Some` at this point.

        let attrs_len = self.header.as_ref().unwrap().2;
        let total_len = (HEADER_LEN + attrs_len) as usize;

        // Wait for the entire message to be available.
        if src.len() < total_len {
            return Ok(None);
        }

        let attrs_buf = &src[HEADER_LEN as usize..total_len];
        let attributes = match many0(parse_attribute)(attrs_buf) {
            Ok((rest, _)) if !rest.is_empty() => {
                self.header = None;
                return Ok(Some(None));
            }
            Ok((_, attrs)) => attrs,
            Err(_) => {
                self.header = None;
                return Ok(Some(None));
            }
        };

        src.advance(total_len);
        let (class, method, _, transaction_id) = self.header.take().unwrap();
        Ok(Some(Some(Message {
            class,
            method,
            transaction_id,
            attributes,
        })))
    }
}

fn parse_header_streaming(
    input: &[u8],
    max_len: u16,
) -> IResult<&[u8], (Class, Method, u16, TransactionId)> {
    use nom::{
        bytes::streaming::{tag, take},
        number::streaming::be_u16,
    };

    let (rest, (class, method)) = bits(parse_class_and_method_streaming)(input)?;

    // The message length MUST contain the size, in bytes, of the message not
    // including the 20-byte STUN header. Since all STUN attributes are padded
    // to a multiple of 4 bytes, the last 2 bits of this field are always zero.
    let (rest, len) = verify(be_u16, |&len| len % 4 == 0 && len <= max_len)(rest)?;

    // The magic cookie field MUST contain the fixed value 0x2112A442 in network
    // byte order.
    let (rest, _) = tag(MAGIC_COOKIE)(rest)?;

    // The transaction ID is a 96-bit identifier, used to uniquely identify STUN
    // transactions.
    let (rest, tr_id) = take(12usize)(rest)?;
    let tr_id = TransactionId::new(tr_id.try_into().unwrap());

    Ok((rest, (class, method, len, tr_id)))
}

fn parse_class_and_method_streaming(
    input: (&[u8], usize),
) -> IResult<(&[u8], usize), (Class, Method)> {
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
    let (rest, unpadded_len) = verify(be_u16, |x| *x <= RawAttribute::MAX_LEN)(rest)?;

    // Since STUN aligns attributes on 32-bit boundaries, attributes whose
    // content is not a multiple of 4 bytes are padded with 1, 2, or 3 bytes of
    // padding so that its value contains a multiple of 4 bytes. The padding
    // bits are ignored, and may be any value.
    let padded_len = (unpadded_len + 3) & !0b11;
    let (rest, value) = take(padded_len)(rest)?;
    let value = Vec::from(&value[..unpadded_len as usize]);

    // OK to unwrap because we already verified the length above.
    let attr = RawAttribute::new(r#type, value).unwrap();

    Ok((rest, attr))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{message::attribute::XorMappedAddress, test_util};

    #[test]
    fn success() {
        for addr in test_util::get_test_addrs() {
            let mut bytes = test_util::new_reference_msg(addr);
            bytes.extend(0..3);

            let mut codec = MessageCodec::new();
            let msg = match codec.decode(&mut bytes) {
                Ok(Some(Some(msg))) => msg,
                _ => panic!("failed to decode"),
            };

            assert_eq!(msg.class, Class::SuccessResponse);
            assert_eq!(msg.method, Method::BINDING);
            assert_eq!(msg.transaction_id, TransactionId::new([3; 12]));
            assert_eq!(msg.attr::<XorMappedAddress>(), Some(XorMappedAddress(addr)));
            assert_eq!(bytes.len(), 3);
            assert!(codec.header.is_none());
        }
    }

    #[test]
    fn incomplete() {
        for addr in test_util::get_test_addrs() {
            let mut bytes = test_util::new_reference_msg(addr);
            let mut codec = MessageCodec::new();

            let len = bytes.len();
            for new_len in (0..len).step_by(3) {
                unsafe {
                    bytes.set_len(new_len);
                }
                match codec.decode(&mut bytes) {
                    Ok(None) => (),
                    _ => panic!("failed to decode incomplete message"),
                };

                assert_eq!(bytes.len(), new_len);
                if new_len >= HEADER_LEN as usize {
                    assert!(codec.header.is_some());
                }
            }

            unsafe {
                bytes.set_len(len);
            }
            let msg = match codec.decode(&mut bytes) {
                Ok(Some(Some(msg))) => msg,
                _ => panic!("failed to eventually decode complete message"),
            };

            assert_eq!(msg.class, Class::SuccessResponse);
            assert_eq!(msg.method, Method::BINDING);
            assert_eq!(msg.transaction_id, TransactionId::new([3; 12]));
            assert_eq!(msg.attr::<XorMappedAddress>(), Some(XorMappedAddress(addr)));
            assert!(bytes.is_empty());
            assert!(codec.header.is_none());
        }
    }

    #[test]
    fn failure() {
        let mut bytes = BytesMut::from(&b"nonsense"[..]);
        let len = bytes.len();

        let mut codec = MessageCodec::new();
        match codec.decode(&mut bytes) {
            Ok(Some(None)) => (),
            x => panic!("failed to decode non-STUN message {:?}", x),
        };

        assert_eq!(bytes.len(), len);
        assert!(codec.header.is_none());
    }
}
