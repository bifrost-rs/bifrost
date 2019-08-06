mod email_address;
mod information;
mod origin;
mod phone_number;
mod session_description;
mod session_name;
mod uri;
mod util;
mod version;

pub use crate::session_description::SessionDescription;

use crate::{
    email_address::EmailAddress, information::Information, origin::Origin,
    phone_number::PhoneNumber, session_name::SessionName, uri::Uri, version::Version,
};

trait Parse {
    fn parse(input: &str) -> nom::IResult<&str, Self>
    where
        Self: Sized;
}

impl<T: Parse> Parse for Option<T> {
    fn parse(input: &str) -> nom::IResult<&str, Self> {
        nom::combinator::opt(T::parse)(input)
    }
}
