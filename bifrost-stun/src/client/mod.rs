mod builder;
pub use self::builder::ClientBuilder;

mod dispatcher;

use std::{io, net::SocketAddr, time::Duration};

use futures::{stream::SplitSink, SinkExt};
use stun_codec::TransactionId;
use tokio_net::udp::UdpFramed;
use tokio_sync::{mpsc, oneshot};
use tokio_timer::Timeout;

use crate::{client::dispatcher::DispatcherMessage, Codec, Message};

pub struct Client {
    sink: SplitSink<UdpFramed<Codec>, (Message, SocketAddr)>,
    peer_addr: SocketAddr,
    dispatcher_tx: mpsc::Sender<DispatcherMessage>,
    rto: Duration,
}

impl Client {
    pub async fn binding(&mut self) -> io::Result<SocketAddr> {
        self.request()
            .await?
            .attributes()
            .find_map(|attr| match attr {
                stun_codec::rfc5389::Attribute::XorMappedAddress(addr) => Some(addr.address()),
                _ => None,
            })
            .ok_or_else(|| {
                io::Error::new(io::ErrorKind::Other, "missing XOR-MAPPED-ADDRESS attribute")
            })
    }

    pub async fn request(&mut self) -> io::Result<Message> {
        let message = new_message();

        let mut num_retries = 0u8;
        let mut rto = self.rto.clone();
        while num_retries < 3 {
            // Add a (id, tx) pair to worker
            let (tx, rx) = oneshot::channel();
            self.dispatcher_tx
                .send(DispatcherMessage::NewTransaction(
                    message.transaction_id(),
                    tx,
                ))
                .await
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

            // Send request to STUN server
            self.sink
                .send((message.clone(), self.peer_addr))
                .await
                .unwrap();

            // Wait for worker to dispatch response
            let rx = Timeout::new(rx, rto);
            match rx.await {
                Ok(Ok(resp)) => return Ok(resp),
                Ok(Err(_)) => return Err(io::Error::new(io::ErrorKind::Other, "recv error")),
                Err(_) => {
                    println!("transaction timed out after {:?}", rto);

                    // Remove current transction from worker
                    self.dispatcher_tx
                        .send(DispatcherMessage::RemoveTransaction(
                            message.transaction_id(),
                        ))
                        .await
                        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

                    num_retries += 1;
                    rto *= 2;
                }
            };
        }

        Err(io::Error::new(
            io::ErrorKind::TimedOut,
            format!("transaction timed out after {} retries", num_retries),
        ))
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        let mut tx = self.dispatcher_tx.clone();
        tokio_executor::spawn(async move {
            let _ = tx.send(DispatcherMessage::Close).await;
        });
    }
}

fn new_message() -> Message {
    use stun_codec::{
        rfc5389::{attributes::Software, methods::BINDING, Attribute},
        MessageClass,
    };

    let mut message = Message::new(MessageClass::Request, BINDING, TransactionId::new([3; 12]));
    message.add_attribute(Attribute::Software(
        Software::new("foo".to_owned()).unwrap(),
    ));
    message
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ClientBuilder;

    #[test]
    fn test_too() {
        tokio_test::block_on(async {
            let mut client = ClientBuilder::new()
                .bind("0.0.0.0:0")
                // .connect("127.0.0.1:3478")
                .connect("stun.l.google.com:19302")
                .rto(Duration::from_secs(5))
                .build()
                .await
                .unwrap();

            let addr = client.binding().await.unwrap();
            println!("My addr: {:?}", addr);
        });
    }
}
