use nom::bytes::complete::{is_not, tag};
use nom::character::complete::line_ending;
use nom::IResult;

// https://tools.ietf.org/html/rfc4566#section-5.3
// s=<session name>
#[derive(Debug, PartialEq)]
pub struct SessionName(pub String);

impl SessionName {
  pub fn parse(input: &str) -> IResult<&str, SessionName> {
    let (input, _) = tag("s=")(input)?;
    let (input, session_name) = is_not("\r\n")(input)?;
    let (input, _) = line_ending(input)?;
    Ok((input, SessionName(session_name.to_owned())))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_valid_1() {
    let s = "s=test\r\nrest\n";
    let (rest, SessionName(session_name)) = SessionName::parse(s).unwrap();
    assert_eq!(rest, "rest\n");
    assert_eq!(session_name, "test");
  }

  #[test]
  fn test_valid_2() {
    let s = "s= \nmore\r\n";
    let (rest, SessionName(session_name)) = SessionName::parse(s).unwrap();
    assert_eq!(rest, "more\r\n");
    assert_eq!(session_name, " ");
  }

  #[test]
  fn test_empty() {
    assert!(SessionName::parse("s=\r\n").is_err());
  }
}
