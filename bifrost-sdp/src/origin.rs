use std::borrow::Cow;

use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::IResult;

use crate::{util, Parse};

/// A parsed origin line, defined in
/// [RFC 4566](https://tools.ietf.org/html/rfc4566#section-5.2).
#[derive(Debug, PartialEq)]
pub struct Origin<'a> {
    pub username: Cow<'a, str>,
    pub session_id: u64,
    pub session_version: u64,
    pub network_type: Cow<'a, str>,
    pub address_type: Cow<'a, str>,
    pub unicast_address: Cow<'a, str>,
}

impl<'a> Parse<'a> for Origin<'a> {
    fn parse(input: &'a str) -> IResult<&str, Self> {
        // o=<username> <sess-id> <sess-version> <nettype> <addrtype> <unicast-address>
        let (rest, _) = tag("o=")(input)?;
        let (rest, username) = util::parse_str_field(rest)?;
        let (rest, _) = tag(" ")(rest)?;
        let (rest, session_id) = util::parse_field(rest)?;
        let (rest, _) = tag(" ")(rest)?;
        let (rest, session_version) = util::parse_field(rest)?;
        let (rest, _) = tag(" ")(rest)?;
        let (rest, network_type) = util::parse_str_field(rest)?;
        let (rest, _) = tag(" ")(rest)?;
        let (rest, address_type) = util::parse_str_field(rest)?;
        let (rest, _) = tag(" ")(rest)?;
        let (rest, unicast_address) = util::parse_str_field(rest)?;
        let (rest, _) = line_ending(rest)?;

        Ok((
            rest,
            Self {
                username,
                session_id,
                session_version,
                network_type,
                address_type,
                unicast_address,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_1() {
        let s = "o=- 4858251974351650128 2 IN IP4 127.0.0.1\r\nrest";
        let expected = Origin {
            username: "-".into(),
            session_id: 4_858_251_974_351_650_128,
            session_version: 2,
            network_type: "IN".into(),
            address_type: "IP4".into(),
            unicast_address: "127.0.0.1".into(),
        };

        let (rest, origin) = Origin::parse(s).unwrap();
        assert_eq!(rest, "rest");
        assert_eq!(origin, expected);
    }

    #[test]
    fn test_valid_2() {
        let s = "o=jdoe 2890844526 2890842807 IN IP4 10.47.16.5\nmore\r\nmore";
        let expected = Origin {
            username: "jdoe".into(),
            session_id: 2_890_844_526,
            session_version: 2_890_842_807,
            network_type: "IN".into(),
            address_type: "IP4".into(),
            unicast_address: "10.47.16.5".into(),
        };

        let (rest, origin) = Origin::parse(s).unwrap();
        assert_eq!(rest, "more\r\nmore");
        assert_eq!(origin, expected);
    }
}
