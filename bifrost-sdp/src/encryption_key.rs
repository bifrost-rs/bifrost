use http::Uri;
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::line_ending;
use nom::combinator::map_res;
use nom::IResult;

use crate::Parse;

/// A parsed encryption key line, defined in
/// [RFC 4566](https://tools.ietf.org/html/rfc4566#section-5.12).
#[derive(Debug, PartialEq)]
pub enum EncryptionKey {
    Clear(String),
    Base64(String),
    Uri(Uri),
    Prompt,
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

    #[test]
    fn test_clear() {
        assert_eq!(
            EncryptionKey::parse("k=clear:foo\r\nmore"),
            Ok(("more", EncryptionKey::Clear("foo".to_owned())))
        );
        assert!(EncryptionKey::parse("k=clear\r\nmore").is_err());
        assert!(EncryptionKey::parse("k=clear:\r\nmore").is_err());
    }

    #[test]
    fn test_base64() {
        assert_eq!(
            EncryptionKey::parse("k=base64:foo\r\nmore"),
            Ok(("more", EncryptionKey::Base64("foo".to_owned())))
        );
        assert!(EncryptionKey::parse("k=base64\r\nmore").is_err());
        assert!(EncryptionKey::parse("k=base64:\r\nmore").is_err());
    }

    #[test]
    fn test_uri() {
        let uri_str = "https://example.org/key";
        let uri = uri_str.parse().unwrap();

        assert_eq!(
            EncryptionKey::parse(&format!("k=uri:{}\r\nmore", uri_str)),
            Ok(("more", EncryptionKey::Uri(uri)))
        );
        assert!(EncryptionKey::parse("k=uri\r\nmore").is_err());
        assert!(EncryptionKey::parse("k=uri:\r\nmore").is_err());
        assert!(EncryptionKey::parse("k=uri:!@#$\r\nmore").is_err());
    }

    #[test]
    fn test_prompt() {
        assert_eq!(
            EncryptionKey::parse("k=prompt\r\nmore"),
            Ok(("more", EncryptionKey::Prompt))
        );
        assert!(EncryptionKey::parse("k=prompt:foo\r\nmore").is_err());
        assert!(EncryptionKey::parse("k=prompt:\r\nmore").is_err());
    }

    #[test]
    fn test_invalid() {
        assert!(EncryptionKey::parse("k=foo\r\nmore").is_err());
        assert!(EncryptionKey::parse("k=foo:\r\nmore").is_err());
        assert!(EncryptionKey::parse("k=foo:bar\r\nmore").is_err());
    }
}
