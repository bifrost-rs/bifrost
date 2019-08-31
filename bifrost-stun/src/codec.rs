use bytecodec::{io::IoEncodeExt, Decode, Encode, Eos, Error};
use bytes::BytesMut;
use stun_codec::{Attribute, Message, MessageDecoder, MessageEncoder};
use tokio_codec::{Decoder, Encoder};

use crate::util::BytesMutWriter;

#[allow(dead_code)]
pub struct StunCodec<A: Attribute> {
    encoder: MessageEncoder<A>,
    decoder: MessageDecoder<A>,
}

impl<A: Attribute> StunCodec<A> {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
}

impl<A: Attribute> Default for StunCodec<A> {
    fn default() -> Self {
        Self {
            encoder: MessageEncoder::default(),
            decoder: MessageDecoder::default(),
        }
    }
}

impl<A: Attribute> Encoder for StunCodec<A> {
    type Item = Message<A>;
    type Error = Error;

    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
        self.encoder.start_encoding(item)?;
        self.encoder.encode_all(BytesMutWriter::new(dst))
    }
}

impl<A: Attribute> Decoder for StunCodec<A> {
    type Item = Message<A>;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        self.decoder.decode(&src, Eos::new(false))?;
        if self.decoder.is_idle() {
            Ok(Some(self.decoder.finish_decoding()??))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use bytecodec::EncodeExt;
    use futures::{stream, SinkExt, StreamExt};
    use stun_codec::{
        rfc5389::{attributes::Software, methods::BINDING, Attribute},
        MessageClass, TransactionId,
    };
    use tokio_codec::{FramedRead, FramedWrite};

    use super::*;

    #[test]
    fn test_read() {
        tokio_test::block_on(async {
            let (messages, encoded) = new_test_data(5);

            let codec = StunCodec::<Attribute>::new();
            let framed_read = FramedRead::new(&encoded[..], codec);
            let decoded: Vec<_> = framed_read
                .take(messages.len().try_into().unwrap())
                .collect()
                .await;

            assert_eq!(decoded.len(), messages.len());

            for (d, m) in decoded.iter().zip(messages.iter()) {
                let d = d.as_ref().unwrap();
                assert_eq!(d.class(), m.class());
                assert_eq!(d.method(), m.method());
                assert_eq!(d.transaction_id(), m.transaction_id());
                assert!(d.attributes().eq(m.attributes()));
            }
        });
    }

    #[test]
    fn test_write() {
        tokio_test::block_on(async {
            let (messages, encoded) = new_test_data(5);

            let codec = StunCodec::<Attribute>::new();
            let mut framed_write = FramedWrite::new(vec![], codec);
            let mut message_stream = stream::iter(messages);
            framed_write.send_all(&mut message_stream).await.unwrap();

            let framed_encoded = framed_write.into_inner();
            assert_eq!(framed_encoded, encoded);
        });
    }

    fn new_test_message() -> Message<Attribute> {
        let mut message = Message::new(MessageClass::Request, BINDING, TransactionId::new([3; 12]));
        message.add_attribute(Attribute::Software(
            Software::new("foo".to_owned()).unwrap(),
        ));
        message
    }

    fn encode_messages(messages: &[Message<Attribute>]) -> Vec<u8> {
        let mut encoder = MessageEncoder::new();
        messages
            .iter()
            .map(|m| encoder.encode_into_bytes(m.clone()).unwrap())
            .fold(vec![], |mut result, mut bytes| {
                result.append(&mut bytes);
                result
            })
    }

    fn new_test_data(n: usize) -> (Vec<Message<Attribute>>, Vec<u8>) {
        let messages: Vec<_> = std::iter::repeat_with(new_test_message).take(n).collect();
        let encoded = encode_messages(&messages);
        (messages, encoded)
    }
}
