use nom::IResult;
use std::fmt;

use crate::{
    Attribute, Bandwidth, ConnectionData, EncryptionKey, Information, MediaInformation, Parse,
};

/// A parsed SDP media description, defined in
/// [RFC 4566](https://tools.ietf.org/html/rfc4566#section-5).
#[derive(Clone, Debug, PartialEq)]
pub struct MediaDescription {
    pub media_information: MediaInformation,
    pub media_title: Option<Information>,
    pub connection_data: Option<ConnectionData>,
    pub bandwidths: Vec<Bandwidth>,
    pub encryption_key: Option<EncryptionKey>,
    pub attributes: Vec<Attribute>,
}

impl fmt::Display for MediaDescription {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.media_information.fmt(f)?;
        self.media_title.iter().try_for_each(|x| x.fmt(f))?;
        self.connection_data.iter().try_for_each(|x| x.fmt(f))?;
        self.bandwidths.iter().try_for_each(|x| x.fmt(f))?;
        self.encryption_key.iter().try_for_each(|x| x.fmt(f))?;
        self.attributes.iter().try_for_each(|x| x.fmt(f))
    }
}

impl Parse for MediaDescription {
    fn parse(input: &str) -> IResult<&str, Self> {
        // m=  (media name and transport address)
        // i=* (media title)
        // c=* (connection information -- optional if included at
        //      session level)
        // b=* (zero or more bandwidth information lines)
        // k=* (encryption key)
        // a=* (zero or more media attribute lines)
        let (rest, media_information) = Parse::parse(input)?;
        let (rest, media_title) = Parse::parse(rest)?;
        let (rest, connection_data) = Parse::parse(rest)?;
        let (rest, bandwidths) = Parse::parse(rest)?;
        let (rest, encryption_key) = Parse::parse(rest)?;
        let (rest, attributes) = Parse::parse(rest)?;

        Ok((
            rest,
            Self {
                media_information,
                media_title,
                connection_data,
                bandwidths,
                encryption_key,
                attributes,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use vec1::vec1;

    use super::*;
    use crate::test_util::{assert_err, assert_parse_display};

    #[test]
    fn valid_1() {
        assert_parse_display(
            "m=audio 49170 RTP/AVP 0\r\n",
            "",
            &MediaDescription {
                media_information: MediaInformation {
                    media_type: "audio".to_owned(),
                    port: "49170".to_owned(),
                    proto: "RTP/AVP".to_owned(),
                    formats: vec1!["0".to_owned()],
                },
                media_title: None,
                connection_data: None,
                bandwidths: vec![],
                encryption_key: None,
                attributes: vec![],
            },
            "m=audio 49170 RTP/AVP 0\r\n",
        );
    }

    #[test]
    fn valid_2() {
        assert_parse_display(
            "m=video 51372 RTP/AVP 99\r\n\
             a=rtpmap:99 h263-1998/90000\r\n\
             more\n",
            "more\n",
            &MediaDescription {
                media_information: MediaInformation {
                    media_type: "video".to_owned(),
                    port: "51372".to_owned(),
                    proto: "RTP/AVP".to_owned(),
                    formats: vec1!["99".to_owned()],
                },
                media_title: None,
                connection_data: None,
                bandwidths: vec![],
                encryption_key: None,
                attributes: vec![Attribute {
                    name: "rtpmap".to_owned(),
                    value: Some("99 h263-1998/90000".to_owned()),
                }],
            },
            "m=video 51372 RTP/AVP 99\r\n\
             a=rtpmap:99 h263-1998/90000\r\n",
        );
    }

    #[test]
    fn valid_3() {
        assert_parse_display(
            "m=video 51372 RTP/AVP 99\r\n\
             i=good stuff\r\n\
             c=IN IP4 224.2.1.1/127/3\r\n\
             b=AB:123\r\n\
             b=X-CD:456\r\n\
             k=base64:abc123\r\n\
             a=rtpmap:99 h263-1998/90000\r\n\
             a=foo\r\n\
             rest\n",
            "rest\n",
            &MediaDescription {
                media_information: MediaInformation {
                    media_type: "video".to_owned(),
                    port: "51372".to_owned(),
                    proto: "RTP/AVP".to_owned(),
                    formats: vec1!["99".to_owned()],
                },
                media_title: Some(Information("good stuff".to_owned())),
                connection_data: Some(ConnectionData {
                    network_type: "IN".to_owned(),
                    address_type: "IP4".to_owned(),
                    connection_address: "224.2.1.1/127/3".to_owned(),
                }),
                bandwidths: vec![
                    Bandwidth {
                        experimental: false,
                        bwtype: "AB".to_owned(),
                        bandwidth: 123,
                    },
                    Bandwidth {
                        experimental: true,
                        bwtype: "CD".to_owned(),
                        bandwidth: 456,
                    },
                ],
                encryption_key: Some(EncryptionKey::Base64("abc123".to_owned())),
                attributes: vec![
                    Attribute {
                        name: "rtpmap".to_owned(),
                        value: Some("99 h263-1998/90000".to_owned()),
                    },
                    Attribute {
                        name: "foo".to_owned(),
                        value: None,
                    },
                ],
            },
            "m=video 51372 RTP/AVP 99\r\n\
             i=good stuff\r\n\
             c=IN IP4 224.2.1.1/127/3\r\n\
             b=AB:123\r\n\
             b=X-CD:456\r\n\
             k=base64:abc123\r\n\
             a=rtpmap:99 h263-1998/90000\r\n\
             a=foo\r\n",
        );
    }

    #[test]
    fn valid_4() {
        assert_parse_display(
            "m=video 51372 RTP/AVP 99\r\n\
             c=IN IP4 224.2.1.1/127/3\r\n\
             i=good stuff\r\n\
             b=AB:123\r\n\
             b=X-CD:456\r\n\
             k=base64:abc123\r\n\
             a=rtpmap:99 h263-1998/90000\r\n\
             a=foo\r\n\
             rest\n",
            "i=good stuff\r\n\
             b=AB:123\r\n\
             b=X-CD:456\r\n\
             k=base64:abc123\r\n\
             a=rtpmap:99 h263-1998/90000\r\n\
             a=foo\r\n\
             rest\n",
            &MediaDescription {
                media_information: MediaInformation {
                    media_type: "video".to_owned(),
                    port: "51372".to_owned(),
                    proto: "RTP/AVP".to_owned(),
                    formats: vec1!["99".to_owned()],
                },
                media_title: None,
                connection_data: Some(ConnectionData {
                    network_type: "IN".to_owned(),
                    address_type: "IP4".to_owned(),
                    connection_address: "224.2.1.1/127/3".to_owned(),
                }),
                bandwidths: vec![],
                encryption_key: None,
                attributes: vec![],
            },
            "m=video 51372 RTP/AVP 99\r\n\
             c=IN IP4 224.2.1.1/127/3\r\n",
        );
    }

    #[test]
    fn invalid() {
        assert_err::<MediaDescription>("");
        assert_err::<MediaDescription>("\r\n");
        assert_err::<MediaDescription>("a=foo\r\n");
        assert_err::<MediaDescription>(
            "i=good stuff\r\n\
             c=IN IP4 224.2.1.1/127/3\r\n\
             b=AB:123\r\n\
             b=X-CD:456\r\n\
             k=base64:abc123\r\n\
             a=rtpmap:99 h263-1998/90000\r\n\
             a=foo\r\n\
             rest\n",
        );
    }
}
