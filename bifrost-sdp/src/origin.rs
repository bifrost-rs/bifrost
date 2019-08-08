use nom::bytes::complete::tag;
use nom::IResult;

use crate::util;
use crate::Parse;

/// A parsed origin line, defined in
/// [RFC 4566](https://tools.ietf.org/html/rfc4566#section-5.2).
#[derive(Debug, PartialEq)]
pub struct Origin {
    pub username: String,
    pub session_id: u64,
    pub session_version: u64,
    pub network_type: String,
    pub address_type: String,
    pub unicast_address: String,
}

impl Parse for Origin {
    fn parse(input: &str) -> IResult<&str, Self> {
        // o=<username> <sess-id> <sess-version> <nettype> <addrtype> <unicast-address>
        let (rest, _) = tag("o=")(input)?;
        let (rest, username) = util::parse_field(rest)?;
        let (rest, session_id) = util::parse_field(rest)?;
        let (rest, session_version) = util::parse_field(rest)?;
        let (rest, network_type) = util::parse_field(rest)?;
        let (rest, address_type) = util::parse_field(rest)?;
        let (rest, unicast_address) = util::parse_last_field(rest)?;

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
            username: "-".to_owned(),
            session_id: 4_858_251_974_351_650_128,
            session_version: 2,
            network_type: "IN".to_owned(),
            address_type: "IP4".to_owned(),
            unicast_address: "127.0.0.1".to_owned(),
        };

        let (rest, origin) = Origin::parse(s).unwrap();
        assert_eq!(rest, "rest");
        assert_eq!(origin, expected);
    }

    #[test]
    fn test_valid_2() {
        let s = "o=jdoe 2890844526 2890842807 IN IP4 10.47.16.5\nmore\r\nmore";
        let expected = Origin {
            username: "jdoe".to_owned(),
            session_id: 2_890_844_526,
            session_version: 2_890_842_807,
            network_type: "IN".to_owned(),
            address_type: "IP4".to_owned(),
            unicast_address: "10.47.16.5".to_owned(),
        };

        let (rest, origin) = Origin::parse(s).unwrap();
        assert_eq!(rest, "more\r\nmore");
        assert_eq!(origin, expected);
    }
}
