mod builder;
pub use self::builder::ClientBuilder;

mod dispatcher;

use std::{
    io::{self, Error, ErrorKind},
    net::SocketAddr,
    time::Duration,
};

use futures_util::{stream::SplitSink, SinkExt};
use stun_codec::TransactionId;
use tokio_net::udp::UdpFramed;
use tokio_sync::{mpsc, oneshot};
use tokio_timer::Timeout;

use crate::{client::dispatcher::DispatcherMessage, Codec, Message};

enum Response {
    Ok(Message),
    TimeOut,
}

pub struct Client {
    sink: SplitSink<UdpFramed<Codec>, (Message, SocketAddr)>,
    peer_addr: SocketAddr,
    dispatcher_tx: mpsc::Sender<DispatcherMessage>,
    rto: Duration,
    max_attempts: u32,
    last_timeout: u32,
}

impl Client {
    pub async fn binding(&mut self) -> io::Result<SocketAddr> {
        self.request()
            .await?
            .attributes()
            .find_map(|attr| {
                if let stun_codec::rfc5389::Attribute::XorMappedAddress(addr) = attr {
                    Some(addr.address())
                } else {
                    None
                }
            })
            .ok_or_else(|| Error::new(ErrorKind::Other, "missing XOR-MAPPED-ADDRESS attribute"))
    }

    pub async fn request(&mut self) -> io::Result<Message> {
        let req = new_message();
        let mut rto = self.rto;

        for attempt in 0..self.max_attempts {
            let resp = self.request_once(req.clone(), rto).await?;

            if let Response::Ok(msg) = resp {
                return Ok(msg);
            }

            println!("request timed out after {:?}", rto);

            // Remove current transction from worker
            self.dispatcher_tx
                .send(DispatcherMessage::RemoveTransaction(req.transaction_id()))
                .await
                .map_err(|e| Error::new(ErrorKind::Other, e))?;

            if attempt + 1 == self.max_attempts - 1 {
                rto = self.rto * self.last_timeout
            } else {
                rto *= 2
            }
        }

        Err(Error::new(
            ErrorKind::TimedOut,
            format!("transaction timed out after {} attempts", self.max_attempts),
        ))
    }

    async fn request_once(&mut self, req: Message, timeout: Duration) -> io::Result<Response> {
        // Add a new transaction to dispatcher
        let (tx, rx) = oneshot::channel();
        self.dispatcher_tx
            .send(DispatcherMessage::NewTransaction(req.transaction_id(), tx))
            .await
            .map_err(|e| Error::new(ErrorKind::Other, e))?;

        // Send out the request
        self.sink
            .send((req, self.peer_addr))
            .await
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;

        // Wait for dispatcher to finish transaction
        match Timeout::new(rx, timeout).await {
            Ok(Ok(resp)) => Ok(Response::Ok(resp)),
            Ok(Err(_)) => Err(Error::new(ErrorKind::Other, "recv error")),
            Err(_) => Ok(Response::TimeOut),
        }
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
