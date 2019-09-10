use crate::{util, Parse};
use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, line_ending},
    IResult,
};
use std::fmt;

/// A parsed bandwidth line, defined in
/// [RFC 4566](https://tools.ietf.org/html/rfc4566#section-5.8).
#[derive(Clone, Debug, PartialEq)]
pub struct Bandwidth {
    pub experimental: bool,
    pub bwtype: String,
    pub bandwidth: u64,
}

impl fmt::Display for Bandwidth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.experimental {
            writeln!(f, "b=X-{}:{}\r", self.bwtype, self.bandwidth)
        } else {
            writeln!(f, "b={}:{}\r", self.bwtype, self.bandwidth)
        }
    }
}

impl Parse for Bandwidth {
    fn parse(input: &str) -> IResult<&str, Self> {
        // b=<bwtype>:<bandwidth>
        let (rest, _) = tag("b=")(input)?;

        let experimental = rest.starts_with("X-");
        let rest = if experimental { &rest[2..] } else { rest };

        let (rest, bwtype) = alphanumeric1(rest)?;
        let (rest, _) = tag(":")(rest)?;

        let (rest, bandwidth) = util::try_parse_field(rest)?;
        let (rest, _) = line_ending(rest)?;

        Ok((
            rest,
            Self {
                experimental,
                bwtype: bwtype.to_owned(),
                bandwidth,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::{assert_err, assert_parse_display};

    #[test]
    fn valid() {
        assert_parse_display(
            "b=CT:256\r\nmore\r\n",
            "more\r\n",
            &Bandwidth {
                experimental: false,
                bwtype: "CT".to_owned(),
                bandwidth: 256,
            },
            "b=CT:256\r\n",
        );
    }

    #[test]
    fn experimental() {
        assert_parse_display(
            "b=X-AB:512\r\n",
            "",
            &Bandwidth {
                experimental: true,
                bwtype: "AB".to_owned(),
                bandwidth: 512,
            },
            "b=X-AB:512\r\n",
        );
    }

    #[test]
    fn invalid() {
        assert_err::<Bandwidth>("b=A-AB:512\r\n");
        assert_err::<Bandwidth>("b=AB:foo\r\n");
    }
}
