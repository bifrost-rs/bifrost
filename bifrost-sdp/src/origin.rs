use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{digit1, line_ending};
use nom::combinator::{map, map_res};
use nom::IResult;

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
    fn parse(input: &str) -> IResult<&str, Origin> {
        // o=<username> <sess-id> <sess-version> <nettype> <addrtype> <unicast-address>
        let (rest, _) = tag("o=")(input)?;

        let (rest, username) = parse_field(rest)?;
        let (rest, _) = tag(" ")(rest)?;

        let (rest, session_id) = parse_u64(rest)?;
        let (rest, _) = tag(" ")(rest)?;

        let (rest, session_version) = parse_u64(rest)?;
        let (rest, _) = tag(" ")(rest)?;

        let (rest, network_type) = parse_field(rest)?;
        let (rest, _) = tag(" ")(rest)?;

        let (rest, address_type) = parse_field(rest)?;
        let (rest, _) = tag(" ")(rest)?;

        let (rest, unicast_address) = parse_field(rest)?;
        let (rest, _) = line_ending(rest)?;

        Ok((
            rest,
            Origin {
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

fn parse_field(input: &str) -> IResult<&str, String> {
    map(is_not(" \r\n"), String::from)(input)
}

fn parse_u64(input: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_1() {
        let s = "o=- 4858251974351650128 2 IN IP4 127.0.0.1\r\nrest";
        let expected = Origin {
            username: "-".to_owned(),
            session_id: 4858251974351650128,
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
            session_id: 2890844526,
            session_version: 2890842807,
            network_type: "IN".to_owned(),
            address_type: "IP4".to_owned(),
            unicast_address: "10.47.16.5".to_owned(),
        };

        let (rest, origin) = Origin::parse(s).unwrap();
        assert_eq!(rest, "more\r\nmore");
        assert_eq!(origin, expected);
    }
}
