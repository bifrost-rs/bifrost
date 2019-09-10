use crate::{util, Parse};
use nom::{combinator::map, IResult};
use std::fmt;

/// A parsed email address line, defined in
/// [RFC 4566](https://tools.ietf.org/html/rfc4566#section-5.6).
#[derive(Clone, Debug, PartialEq)]
pub struct EmailAddress(pub String);

impl fmt::Display for EmailAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "e={}\r", self.0)
    }
}

impl Parse for EmailAddress {
    fn parse(input: &str) -> IResult<&str, Self> {
        // e=<email-address>
        map(util::parse_nonempty_line("e="), |value| {
            Self(value.to_owned())
        })(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::{assert_err, assert_parse_display};

    #[test]
    fn valid() {
        let email_addr = "j.doe@example.com (Jane Doe)";
        assert_parse_display(
            &format!("e={}\n\nresto\r\n", email_addr),
            "\nresto\r\n",
            &EmailAddress(email_addr.to_owned()),
            &format!("e={}\r\n", email_addr),
        );
    }

    #[test]
    fn invalid() {
        assert_err::<EmailAddress>("e=\r\n");
        assert_err::<EmailAddress>("x=foo@bar.com\r\n");
    }
}
