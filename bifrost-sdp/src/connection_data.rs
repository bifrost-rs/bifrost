use std::borrow::Cow;

use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::IResult;

use crate::{util, Parse};

/// A parsed connection data line, defined in
/// [RFC 4566](https://tools.ietf.org/html/rfc4566#section-5.7).
#[derive(Debug, PartialEq)]
pub struct ConnectionData<'a> {
    pub network_type: Cow<'a, str>,
    pub address_type: Cow<'a, str>,
    pub connection_address: Cow<'a, str>,
}

impl<'a> Parse<'a> for ConnectionData<'a> {
    fn parse(input: &'a str) -> IResult<&str, Self> {
        // c=<nettype> <addrtype> <connection-address>
        let (rest, _) = tag("c=")(input)?;
        let (rest, network_type) = util::parse_str_field(rest)?;
        let (rest, _) = tag(" ")(rest)?;
        let (rest, address_type) = util::parse_str_field(rest)?;
        let (rest, _) = tag(" ")(rest)?;
        let (rest, connection_address) = util::parse_str_field(rest)?;
        let (rest, _) = line_ending(rest)?;

        Ok((
            rest,
            Self {
                network_type,
                address_type,
                connection_address,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        let s = "c=IN IP4 224.2.1.1/127/3\r\n rest\n";
        let (rest, c) = ConnectionData::parse(s).unwrap();
        assert_eq!(rest, " rest\n");
        assert_eq!(
            c,
            ConnectionData {
                network_type: "IN".into(),
                address_type: "IP4".into(),
                connection_address: "224.2.1.1/127/3".into(),
            }
        )
    }

    #[test]
    fn test_invalid() {
        let s1 = "c=IN IP4\r\n";
        let s2 = "c=IN IP4 224.2.1.1/127/3 foo\r\n";
        assert!(ConnectionData::parse(s1).is_err());
        assert!(ConnectionData::parse(s2).is_err());
    }
}
