// Expose only the `SessionDescription` type.
mod session_description;
pub use self::session_description::SessionDescription;

mod email_address;
use self::email_address::EmailAddress;

mod information;
use self::information::Information;

mod origin;
use self::origin::Origin;

mod phone_number;
use self::phone_number::PhoneNumber;

mod session_name;
use self::session_name::SessionName;

mod uri;
use self::uri::Uri;

mod version;
use self::version::Version;

mod util;

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
