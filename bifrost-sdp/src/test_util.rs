use crate::Parse;
use std::fmt::{Debug, Display};

pub fn assert_parse_display<T: Debug + Display + Parse + PartialEq>(
    input: &str,
    expected_rest: &str,
    expected_parsed: &T,
    expected_display: &str,
) {
    let (rest, parsed) = T::parse(input).unwrap();
    assert_eq!(rest, expected_rest);
    assert_eq!(&parsed, expected_parsed);
    assert_eq!(parsed.to_string(), expected_display);
}

pub fn assert_err<T: Parse>(input: &str) {
    assert!(T::parse(input).is_err());
}
