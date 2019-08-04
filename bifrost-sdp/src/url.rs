use nom::combinator::map_res;
use nom::IResult;
use url::Url;

use crate::util;
use crate::Parse;

impl Parse for Url {
    fn parse(input: &str) -> IResult<&str, Url> {
        map_res(util::parse_single_field_line("u="), Url::parse)(input)
    }
}
