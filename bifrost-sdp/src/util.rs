use nom::bytes::complete::{is_not, tag};
use nom::character::complete::line_ending;
use nom::IResult;

pub fn parse_single_field_line<'a>(
    type_tag: &'a str,
) -> impl Fn(&'a str) -> IResult<&'a str, &'a str> {
    move |input: &'a str| {
        let (rest, _) = tag(type_tag)(input)?;
        let (rest, value) = is_not("\r\n")(rest)?;
        let (rest, _) = line_ending(rest)?;
        Ok((rest, value))
    }
}
