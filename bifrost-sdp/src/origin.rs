use std::fmt;

use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::IResult;

use crate::{util, Parse};

/// A parsed origin line, defined in
/// [RFC 4566](https://tools.ietf.org/html/rfc4566#section-5.2).
#[derive(Clone, Debug, PartialEq)]
pub struct Origin {
    pub username: String,
    pub session_id: u64,
    pub session_version: u64,
    pub network_type: String,
    pub address_type: String,
    pub unicast_address: String,
}

impl fmt::Display for Origin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "o={} {} {} {} {} {}\r",
            self.username,
            self.session_id,
            self.session_version,
            self.network_type,
            self.address_type,
            self.unicast_address
        )
    }
}

impl Parse for Origin {
    fn parse(input: &str) -> IResult<&str, Self> {
        // o=<username> <sess-id> <sess-version> <nettype> <addrtype> <unicast-address>
        let (rest, _) = tag("o=")(input)?;

        let (rest, username) = util::parse_field(rest)?;
        let (rest, _) = tag(" ")(rest)?;

        let (rest, session_id) = util::try_parse_field(rest)?;
        let (rest, _) = tag(" ")(rest)?;

        let (rest, session_version) = util::try_parse_field(rest)?;
        let (rest, _) = tag(" ")(rest)?;

        let (rest, network_type) = util::parse_field(rest)?;
        let (rest, _) = tag(" ")(rest)?;

        let (rest, address_type) = util::parse_field(rest)?;
        let (rest, _) = tag(" ")(rest)?;

        let (rest, unicast_address) = util::parse_field(rest)?;
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
    use super::Origin;
    use crate::test_util::assert_parse_display;

    #[test]
    fn test_valid() {
        assert_parse_display(
            "o=- 4858251974351650128 2 IN IP4 127.0.0.1\r\nrest",
            "rest",
            &Origin {
                username: "-".to_owned(),
                session_id: 4_858_251_974_351_650_128,
                session_version: 2,
                network_type: "IN".to_owned(),
                address_type: "IP4".to_owned(),
                unicast_address: "127.0.0.1".to_owned(),
            },
            "o=- 4858251974351650128 2 IN IP4 127.0.0.1\r\n",
        );

        assert_parse_display(
            "o=jdoe 2890844526 2890842807 IN IP4 10.47.16.5\nmore\r\nmore",
            "more\r\nmore",
            &Origin {
                username: "jdoe".to_owned(),
                session_id: 2_890_844_526,
                session_version: 2_890_842_807,
                network_type: "IN".to_owned(),
                address_type: "IP4".to_owned(),
                unicast_address: "10.47.16.5".to_owned(),
            },
            "o=jdoe 2890844526 2890842807 IN IP4 10.47.16.5\r\n",
        );
    }
}
