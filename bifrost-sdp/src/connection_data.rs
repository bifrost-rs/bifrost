use nom::{bytes::complete::tag, character::complete::line_ending, IResult};
use std::fmt;

use crate::{util, Parse};

/// A parsed connection data line, defined in
/// [RFC 4566](https://tools.ietf.org/html/rfc4566#section-5.7).
#[derive(Clone, Debug, PartialEq)]
pub struct ConnectionData {
    pub network_type: String,
    pub address_type: String,
    pub connection_address: String,
}

impl fmt::Display for ConnectionData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "c={} {} {}\r",
            self.network_type, self.address_type, self.connection_address
        )
    }
}

impl Parse for ConnectionData {
    fn parse(input: &str) -> IResult<&str, Self> {
        // c=<nettype> <addrtype> <connection-address>
        let (rest, _) = tag("c=")(input)?;

        let (rest, network_type) = util::parse_field(rest)?;
        let (rest, _) = tag(" ")(rest)?;

        let (rest, address_type) = util::parse_field(rest)?;
        let (rest, _) = tag(" ")(rest)?;

        let (rest, connection_address) = util::parse_field(rest)?;
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
    use crate::test_util::{assert_err, assert_parse_display};

    #[test]
    fn test_valid() {
        assert_parse_display(
            "c=IN IP4 224.2.1.1/127/3\r\n rest\n",
            " rest\n",
            &ConnectionData {
                network_type: "IN".to_owned(),
                address_type: "IP4".to_owned(),
                connection_address: "224.2.1.1/127/3".to_owned(),
            },
            "c=IN IP4 224.2.1.1/127/3\r\n",
        );
    }

    #[test]
    fn test_invalid() {
        assert_err::<ConnectionData>("c=IN IP4\r\n");
        assert_err::<ConnectionData>("c=IN IP4 224.2.1.1/127/3 foo\r\n");
    }
}
