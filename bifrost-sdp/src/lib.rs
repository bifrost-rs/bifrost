mod information;
mod origin;
mod session_description;
mod session_name;
mod uri;
mod util;
mod version;

pub use crate::information::Information;
pub use crate::origin::Origin;
pub use crate::session_description::SessionDescription;
pub use crate::session_name::SessionName;
pub use crate::uri::Uri;
pub use crate::version::Version;

use nom::IResult;

trait Parse {
    fn parse(input: &str) -> IResult<&str, Self>
    where
        Self: Sized;
}

impl<T: Parse> Parse for Option<T> {
    fn parse(input: &str) -> IResult<&str, Self> {
        nom::combinator::opt(T::parse)(input)
    }
}
