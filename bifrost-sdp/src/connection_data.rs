use nom::bytes::complete::tag;
use nom::IResult;

use crate::util;
use crate::Parse;

/// A parsed connection data line, defined in
/// [RFC 4566](https://tools.ietf.org/html/rfc4566#section-5.7).
#[derive(Debug, PartialEq)]
pub struct ConnectionData {
    pub network_type: String,
    pub address_type: String,
    pub connection_address: String,
}

impl Parse for ConnectionData {
    fn parse(input: &str) -> IResult<&str, Self> {
        // c=<nettype> <addrtype> <connection-address>
        let (rest, _) = tag("c=")(input)?;
        let (rest, network_type) = util::parse_field(rest)?;
        let (rest, address_type) = util::parse_field(rest)?;
        let (rest, connection_address) = util::parse_last_field(rest)?;

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
                network_type: "IN".to_owned(),
                address_type: "IP4".to_owned(),
                connection_address: "224.2.1.1/127/3".to_owned(),
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
