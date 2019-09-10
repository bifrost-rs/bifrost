use nom::{
    bytes::complete::{tag, take},
    combinator::verify,
    number::complete::{be_u16, be_u8},
    IResult,
};
use std::net::{IpAddr, SocketAddr};

use crate::message::{attribute::Attribute, RawAttribute, TransactionId, MAGIC_COOKIE};

/// The XOR-MAPPED-ADDRESS attribute, defined in
/// [RFC 5389](https://tools.ietf.org/html/rfc5389#section-15.2).
#[derive(Debug, Eq, PartialEq)]
pub struct XorMappedAddress(pub SocketAddr);

impl Attribute for XorMappedAddress {
    const TYPE: u16 = 0x0020;

    fn from_raw(raw: &[u8], tr_id: &TransactionId) -> Option<Self> {
        parse(raw, tr_id)
            .ok()
            .and_then(|(rest, attr)| if rest.is_empty() { Some(attr) } else { None })
    }

    // TODO: Add tests
    fn to_raw(&self, tr_id: &TransactionId) -> RawAttribute {
        let is_ipv4 = self.0.is_ipv4();
        let len: u16 = if is_ipv4 { 8 } else { 20 };
        let mut buf = Vec::with_capacity(len as usize);

        buf.push(0);
        buf.push(if is_ipv4 { 1 } else { 2 });

        let x_port = self.0.port() ^ u16::from_be_bytes([MAGIC_COOKIE[0], MAGIC_COOKIE[1]]);
        buf.extend(&x_port.to_be_bytes());

        match self.0.ip() {
            IpAddr::V4(addr) => {
                let bytes = xor_4_bytes(&addr.octets(), MAGIC_COOKIE);
                buf.extend(&bytes);
            }
            IpAddr::V6(addr) => {
                let mut xor_bytes = [0u8; 16];
                xor_bytes[..4].copy_from_slice(&MAGIC_COOKIE);
                xor_bytes[4..].copy_from_slice(tr_id.as_bytes());

                let bytes = xor_16_bytes(&addr.octets(), xor_bytes);

                buf.extend(&bytes);
            }
        }

        RawAttribute::new(Self::TYPE, buf).unwrap()
    }
}

fn parse<'a>(input: &'a [u8], tr_id: &TransactionId) -> IResult<&'a [u8], XorMappedAddress> {
    // The first 8 bits of the XOR_MAPPED-ADDRESS MUST be set to 0 and MUST be
    // ignored by receivers.
    let (rest, _) = tag([0])(input)?;

    // The 8-bit address family can take on the following values:
    //
    //   0x01:IPv4
    //   0x02:IPv6
    let (rest, family) = verify(be_u8, |&x| x == 1 || x == 2)(rest)?;

    // X-Port is computed by taking the mapped port in host byte order,
    // XOR'ing it with the most significant 16 bits of the magic cookie, and
    // then the converting the result to network byte order.
    let (rest, mut port) = be_u16(rest)?;
    port ^= u16::from_be_bytes([MAGIC_COOKIE[0], MAGIC_COOKIE[1]]);

    // If the IP address family is IPv4, X-Address is computed by taking the
    // mapped IP address in host byte order, XOR'ing it with the magic cookie,
    // and converting the result to network byte order.  If the IP address
    // family is IPv6, X-Address is computed by taking the mapped IP address in
    // host byte order, XOR'ing it with the concatenation of the magic cookie
    // and the 96-bit transaction ID, and converting the result to network byte
    // order.
    let (rest, ip_addr) = if family == 1 {
        let (rest, x_addr) = take(4usize)(rest)?;

        (rest, IpAddr::from(xor_4_bytes(x_addr, MAGIC_COOKIE)))
    } else {
        let (rest, x_addr) = take(16usize)(rest)?;

        let mut xor_bytes = [0u8; 16];
        xor_bytes[..4].copy_from_slice(&MAGIC_COOKIE);
        xor_bytes[4..].copy_from_slice(tr_id.as_bytes());

        (rest, IpAddr::from(xor_16_bytes(x_addr, xor_bytes)))
    };

    Ok((rest, XorMappedAddress(SocketAddr::new(ip_addr, port))))
}

fn xor_4_bytes(a: &[u8], mut b: [u8; 4]) -> [u8; 4] {
    assert_eq!(a.len(), b.len());

    for i in 0..b.len() {
        b[i] ^= a[i];
    }
    b
}

fn xor_16_bytes(a: &[u8], mut b: [u8; 16]) -> [u8; 16] {
    assert_eq!(a.len(), b.len());

    for i in 0..b.len() {
        b[i] ^= a[i];
    }
    b
}

#[cfg(test)]
mod tests {
    use super::*;

    const TR_ID: TransactionId = TransactionId::new([
        0x8f, 0x18, 0x41, 0x1, 0x16, 0x70, 0x5f, 0xea, 0xec, 0x0b, 0xa7, 0xab,
    ]);

    #[test]
    fn xor_4() {
        let a = [1, 2, 3, 4];
        let b = [5, 6, 7, 8];
        let c = xor_4_bytes(&a, b);
        assert_eq!(c, [1 ^ 5, 2 ^ 6, 3 ^ 7, 4 ^ 8]);
    }

    #[test]
    fn xor_16() {
        let a: Vec<_> = (1..=16).collect();

        let mut b = [0u8; 16];
        b.copy_from_slice(&(17..=32).collect::<Vec<_>>());

        let c = xor_16_bytes(&a, b);

        let mut expected = [0u8; 16];
        expected.copy_from_slice(
            &(1..=16)
                .zip(17..=32)
                .map(|(x, y)| x ^ y)
                .collect::<Vec<_>>(),
        );

        assert_eq!(c, expected);
    }

    #[test]
    fn ipv4() {
        let raw = [0x00, 0x01, 0x9c, 0xd5, 0xf4, 0x9f, 0x38, 0xae];
        let attr = XorMappedAddress::from_raw(&raw, &TR_ID).unwrap();
        let expected = XorMappedAddress("213.141.156.236:48583".parse().unwrap());
        assert_eq!(attr, expected);
    }

    #[test]
    fn ipv6() {
        let raw = [
            0x00, 0x02, 0x20, 0xa9, 0x01, 0x13, 0xa9, 0xfa, 0x0a, 0xbb, 0x49, 0xd2, 0x05, 0x69,
            0xd5, 0xc4, 0xef, 0x7b, 0xd4, 0xe3,
        ];
        let attr = XorMappedAddress::from_raw(&raw, &TR_ID).unwrap();
        let expected = XorMappedAddress(
            "[2001:db8:85a3:8d3:1319:8a2e:370:7348]:443"
                .parse()
                .unwrap(),
        );
        assert_eq!(attr, expected);
    }

    #[test]
    fn nonzero_prefix() {
        let raw = [0x42, 0x01, 0x9c, 0xd5, 0xf4, 0x9f, 0x38, 0xae];
        assert!(XorMappedAddress::from_raw(&raw, &TR_ID).is_none());
    }

    #[test]
    fn invalid_family() {
        let raw = [0x00, 0x03, 0x9c, 0xd5, 0xf4, 0x9f, 0x38, 0xae];
        assert!(XorMappedAddress::from_raw(&raw, &TR_ID).is_none());
    }

    #[test]
    fn invalid_ipv4() {
        let raw1 = [0x00, 0x01, 0x9c, 0xd5, 0xf4, 0x9f, 0x38];
        let raw2 = [0x00, 0x01, 0x9c, 0xd5, 0xf4, 0x9f, 0x38, 0xae, 0x42];
        assert!(XorMappedAddress::from_raw(&raw1, &TR_ID).is_none());
        assert!(XorMappedAddress::from_raw(&raw2, &TR_ID).is_none());
    }

    #[test]
    fn invalid_ipv6() {
        let raw1 = [
            0x00, 0x02, 0x20, 0xa9, 0x01, 0x13, 0xa9, 0xfa, 0x0a, 0xbb, 0x49, 0xd2, 0x05, 0x69,
            0xd5, 0xc4, 0xef, 0x7b, 0xd4,
        ];
        let raw2 = [
            0x00, 0x02, 0x20, 0xa9, 0x01, 0x13, 0xa9, 0xfa, 0x0a, 0xbb, 0x49, 0xd2, 0x05, 0x69,
            0xd5, 0xc4, 0xef, 0x7b, 0xd4, 0xe3, 0x42,
        ];
        assert!(XorMappedAddress::from_raw(&raw1, &TR_ID).is_none());
        assert!(XorMappedAddress::from_raw(&raw2, &TR_ID).is_none());
    }
}
