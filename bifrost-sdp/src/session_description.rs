use nom::IResult;

use crate::{
    Bandwidth, ConnectionData, EmailAddress, Information, Origin, Parse, PhoneNumber, SessionName,
    Uri, Version,
};

/// A parsed SDP session description, defined in
/// [RFC 4566](https://tools.ietf.org/html/rfc4566#section-5).
#[derive(Debug, PartialEq)]
pub struct SessionDescription {
    pub version: Version,
    pub origin: Origin,
    pub session_name: SessionName,
    pub session_information: Option<Information>,
    pub uri: Option<Uri>,
    pub email_address: Option<EmailAddress>,
    pub phone_number: Option<PhoneNumber>,
    pub connection_data: Option<ConnectionData>,
    pub bandwidth: Option<Bandwidth>,
}

impl Parse for SessionDescription {
    fn parse(input: &str) -> IResult<&str, Self> {
        // v=  (protocol version)
        // o=  (originator and session identifier)
        // s=  (session name)
        // i=* (session information)
        // u=* (URI of description)
        // e=* (email address)
        // p=* (phone number)
        // c=* (connection information -- not required if included in all media)
        // b=* (zero or more bandwidth information lines)
        // One or more time descriptions ("t=" and "r=" lines; see below)
        // z=* (time zone adjustments)
        // k=* (encryption key)
        // a=* (zero or more session attribute lines)
        // Zero or more media descriptions

        let (rest, version) = Parse::parse(input)?;
        let (rest, origin) = Parse::parse(rest)?;
        let (rest, session_name) = Parse::parse(rest)?;
        let (rest, session_information) = Parse::parse(rest)?;
        let (rest, uri) = Parse::parse(rest)?;
        let (rest, email_address) = Parse::parse(rest)?;
        let (rest, phone_number) = Parse::parse(rest)?;
        let (rest, connection_data) = Parse::parse(rest)?;
        let (rest, bandwidth) = Parse::parse(rest)?;

        Ok((
            rest,
            Self {
                version,
                origin,
                session_name,
                session_information,
                uri,
                email_address,
                phone_number,
                connection_data,
                bandwidth,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        let s = r#"v=0
o=jdoe 2890844526 2890842807 IN IP4 10.47.16.5
s=SDP Seminar
i=A Seminar on the session description protocol
u=http://www.example.com/seminars/sdp.pdf
e=j.doe@example.com (Jane Doe)
c=IN IP4 224.2.36.42/127
b=X-YZ:128
"#;

        let expected = SessionDescription {
            version: Version,
            origin: Origin {
                username: "jdoe".to_owned(),
                session_id: 2_890_844_526,
                session_version: 2_890_842_807,
                network_type: "IN".to_owned(),
                address_type: "IP4".to_owned(),
                unicast_address: "10.47.16.5".to_owned(),
            },
            session_name: SessionName("SDP Seminar".to_owned()),
            session_information: Some(Information(
                "A Seminar on the session description protocol".to_owned(),
            )),
            uri: Some(Uri("http://www.example.com/seminars/sdp.pdf"
                .parse()
                .unwrap())),
            email_address: Some(EmailAddress("j.doe@example.com (Jane Doe)".to_owned())),
            phone_number: None,
            connection_data: Some(ConnectionData {
                network_type: "IN".to_owned(),
                address_type: "IP4".to_owned(),
                connection_address: "224.2.36.42/127".to_owned(),
            }),
            bandwidth: Some(Bandwidth {
                experimental: true,
                bwtype: "YZ".to_owned(),
                bandwidth: 128,
            }),
        };

        let (_, sdp) = SessionDescription::parse(s).unwrap();
        assert_eq!(sdp, expected);
    }
}
