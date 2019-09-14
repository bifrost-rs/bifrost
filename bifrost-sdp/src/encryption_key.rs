use crate::Parse;
use http::Uri;
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::line_ending;
use nom::combinator::map_res;
use nom::IResult;
use std::fmt;

/// A parsed encryption key line, defined in
/// [RFC 4566](https://tools.ietf.org/html/rfc4566#section-5.12).
#[derive(Clone, Debug, PartialEq)]
pub enum EncryptionKey {
    Clear(String),
    Base64(String),
    Uri(Uri),
    Prompt,
}

impl fmt::Display for EncryptionKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Clear(key) => writeln!(f, "k=clear:{}\r", key),
            Self::Base64(key) => writeln!(f, "k=base64:{}\r", key),
            Self::Uri(uri) => writeln!(f, "k=uri:{}\r", uri),
            Self::Prompt => writeln!(f, "k=prompt\r"),
        }
    }
}

impl Parse for EncryptionKey {
    fn parse(input: &str) -> IResult<&str, Self> {
        // k=<method>
        // k=<method>:<encryption key>
        let (rest, _) = tag("k=")(input)?;
        let (rest, key) = alt((parse_clear, parse_base64, parse_uri, parse_prompt))(rest)?;
        let (rest, _) = line_ending(rest)?;
        Ok((rest, key))
    }
}

fn parse_clear(input: &str) -> IResult<&str, EncryptionKey> {
    let (rest, _) = tag("clear:")(input)?;
    let (rest, key) = is_not("\r\n")(rest)?;
    Ok((rest, EncryptionKey::Clear(key.to_owned())))
}

fn parse_base64(input: &str) -> IResult<&str, EncryptionKey> {
    let (rest, _) = tag("base64:")(input)?;
    let (rest, key) = is_not("\r\n")(rest)?;
    Ok((rest, EncryptionKey::Base64(key.to_owned())))
}

fn parse_uri(input: &str) -> IResult<&str, EncryptionKey> {
    let (rest, _) = tag("uri:")(input)?;
    let (rest, key) = map_res(is_not("\r\n"), str::parse)(rest)?;
    Ok((rest, EncryptionKey::Uri(key)))
}

fn parse_prompt(input: &str) -> IResult<&str, EncryptionKey> {
    let (rest, _) = tag("prompt")(input)?;
    Ok((rest, EncryptionKey::Prompt))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::{assert_err, assert_parse_display};

    #[test]
    fn clear() {
        assert_parse_display(
            "k=clear:foo\r\nmore",
            "more",
            &EncryptionKey::Clear("foo".to_owned()),
            "k=clear:foo\r\n",
        );
        assert_err::<EncryptionKey>("k=clear\r\nmore");
        assert_err::<EncryptionKey>("k=clear:\r\nmore");
    }

    #[test]
    fn base64() {
        assert_parse_display(
            "k=base64:foo\r\n\rmore",
            "\rmore",
            &EncryptionKey::Base64("foo".to_owned()),
            "k=base64:foo\r\n",
        );
        assert_err::<EncryptionKey>("k=base64\r\nmore");
        assert_err::<EncryptionKey>("k=base64:\r\nmore");
    }

    #[test]
    fn uri() {
        let uri_str = "https://example.org/key";
        let uri = uri_str.parse().unwrap();

        assert_parse_display(
            &format!("k=uri:{}\r\n\nmore\r", uri_str),
            "\nmore\r",
            &EncryptionKey::Uri(uri),
            &format!("k=uri:{}\r\n", uri_str),
        );
        assert_err::<EncryptionKey>("k=uri\r\nmore");
        assert_err::<EncryptionKey>("k=uri:\r\nmore");
        assert_err::<EncryptionKey>("k=uri:!@#$\r\nmore");
    }

    #[test]
    fn prompt() {
        assert_parse_display(
            "k=prompt\r\nmore",
            "more",
            &EncryptionKey::Prompt,
            "k=prompt\r\n",
        );
        assert_err::<EncryptionKey>("k=prompt:foo\r\nmore");
        assert_err::<EncryptionKey>("k=prompt:\r\nmore");
    }

    #[test]
    fn invalid() {
        assert_err::<EncryptionKey>("k=foo\r\nmore");
        assert_err::<EncryptionKey>("k=foo:\r\nmore");
        assert_err::<EncryptionKey>("k=foo:bar\r\nmore");
    }
}
