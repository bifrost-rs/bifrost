use std::{io, net::SocketAddr, time::Duration};

use futures::{future, stream, Stream, StreamExt};
use tokio_net::{
    udp::{UdpFramed, UdpSocket},
    ToSocketAddrs,
};
use tokio_sync::mpsc;

use crate::{
    client::{
        dispatcher::{Dispatcher, DispatcherMessage},
        Client,
    },
    Codec, Message,
};

pub struct ClientBuilder<L, P> {
    local_addr: L,
    peer_addr: P,
    rto: Duration,
}

impl Default for ClientBuilder<(), ()> {
    fn default() -> Self {
        Self {
            local_addr: (),
            peer_addr: (),
            rto: Duration::from_millis(500),
        }
    }
}

impl ClientBuilder<(), ()> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn bind<L: ToSocketAddrs>(self, local_addr: L) -> ClientBuilder<L, ()> {
        ClientBuilder {
            local_addr,
            peer_addr: (),
            rto: self.rto,
        }
    }
}

impl<L> ClientBuilder<L, ()>
where
    L: ToSocketAddrs,
{
    pub fn connect<P: ToSocketAddrs>(self, peer_addr: P) -> ClientBuilder<L, P> {
        ClientBuilder {
            local_addr: self.local_addr,
            peer_addr,
            rto: self.rto,
        }
    }
}

impl<L, P> ClientBuilder<L, P> {
    pub fn rto(self, rto: Duration) -> ClientBuilder<L, P> {
        ClientBuilder {
            local_addr: self.local_addr,
            peer_addr: self.peer_addr,
            rto,
        }
    }
}

impl<L, P> ClientBuilder<L, P>
where
    L: ToSocketAddrs,
    P: ToSocketAddrs,
{
    pub async fn build(self) -> io::Result<Client> {
        let mut socket = UdpSocket::bind(self.local_addr).await?;
        let peer_addr = resolve_peer_addr(&mut socket, self.peer_addr).await?;

        let framed = UdpFramed::new(socket, Codec::new());
        let (sink, stream) = framed.split();

        let (dispatcher_tx, dispatcher_rx) = mpsc::channel(100);

        // Merge dispatcher_tx with incoming UDP stream
        let udp_stream = filter_addr(stream, peer_addr);
        let dispatcher_rx = stream::select(dispatcher_rx, udp_stream);

        tokio_executor::spawn(async move {
            let mut dispatcher = Dispatcher::new(dispatcher_rx);
            dispatcher.run().await;
        });

        Ok(Client {
            sink,
            peer_addr,
            dispatcher_tx,
            rto: self.rto,
        })
    }
}

async fn resolve_peer_addr<A: ToSocketAddrs>(
    socket: &mut UdpSocket,
    addr: A,
) -> io::Result<SocketAddr> {
    for peer_addr in addr.to_socket_addrs().await? {
        if socket.send_to(&[0], peer_addr).await.is_ok() {
            return Ok(peer_addr);
        }
    }
    Err(io::Error::new(
        io::ErrorKind::InvalidInput,
        "no addresses to send data to",
    ))
}

fn filter_addr(
    st: impl Stream<Item = Result<(Message, SocketAddr), bytecodec::Error>>,
    addr: SocketAddr,
) -> impl Stream<Item = DispatcherMessage> {
    st.filter_map(move |x| {
        future::ready(match x {
            Ok((_, a)) if a == addr => Some(DispatcherMessage::Recv(Ok(x.unwrap().0))),
            Ok(_) => None,
            Err(e) => Some(DispatcherMessage::Recv(Err(e))),
        })
    })
}
