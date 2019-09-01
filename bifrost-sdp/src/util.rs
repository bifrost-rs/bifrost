use nom::{
    bytes::complete::{is_not, tag},
    character::complete::line_ending,
    combinator::{map, map_res},
    IResult,
};
use std::str::FromStr;

/// Parses the input until a whitespace or a newline.
pub fn parse_field<'a, T: From<&'a str>>(input: &'a str) -> IResult<&str, T> {
    map(is_not(" \r\n"), T::from)(input)
}

/// Parses the input until a whitespace or a newline, and tries to convert the
/// parsed string to a `T`.
pub fn try_parse_field<T: FromStr>(input: &str) -> IResult<&str, T> {
    map_res(is_not(" \r\n"), FromStr::from_str)(input)
}

pub fn parse_nonempty_line(type_tag: &str) -> impl Fn(&str) -> IResult<&str, &str> + '_ {
    move |input| {
        let (rest, _) = tag(type_tag)(input)?;
        let (rest, value) = is_not("\r\n")(rest)?;
        let (rest, _) = line_ending(rest)?;
        Ok((rest, value))
    }
}
