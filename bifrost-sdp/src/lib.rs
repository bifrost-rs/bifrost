mod bandwidth;
mod connection_data;
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
    bandwidth::Bandwidth, connection_data::ConnectionData, email_address::EmailAddress,
    information::Information, origin::Origin, phone_number::PhoneNumber, session_name::SessionName,
    uri::Uri, version::Version,
};

trait Parse: Sized {
    fn parse(input: &str) -> nom::IResult<&str, Self>;
}

impl<T: Parse> Parse for Option<T> {
    fn parse(input: &str) -> nom::IResult<&str, Self> {
        nom::combinator::opt(T::parse)(input)
    }
}
