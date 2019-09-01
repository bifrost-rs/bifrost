use futures_core::Stream;
use futures_util::StreamExt;
use std::collections::HashMap;
use stun_codec::TransactionId;
use tokio_sync::oneshot;

use crate::Message;

pub enum DispatcherMessage {
    NewTransaction(TransactionId, oneshot::Sender<Message>),
    RemoveTransaction(TransactionId),
    Recv(Result<Message, bytecodec::Error>),
    Close,
}

pub struct Dispatcher<St> {
    transactions: HashMap<TransactionId, oneshot::Sender<Message>>,
    rx: St,
}

impl<St> Dispatcher<St>
where
    St: Stream<Item = DispatcherMessage> + Unpin,
{
    pub fn new(rx: St) -> Self {
        Self {
            transactions: HashMap::new(),
            rx,
        }
    }

    pub async fn run(&mut self) {
        while let Some(item) = self.rx.next().await {
            match item {
                DispatcherMessage::NewTransaction(id, tx) => {
                    self.transactions.insert(id, tx);
                }
                DispatcherMessage::RemoveTransaction(id) => {
                    self.transactions.remove(&id);
                }
                DispatcherMessage::Recv(msg) => {
                    self.handle_recv(msg);
                }
                DispatcherMessage::Close => break,
            }
        }
    }

    fn handle_recv(&mut self, msg: Result<Message, bytecodec::Error>) {
        if let Ok(msg) = msg {
            if let Some(tx) = self.transactions.remove(&msg.transaction_id()) {
                tokio_executor::spawn(async move {
                    tokio_timer::sleep(std::time::Duration::from_secs(3)).await;
                    let _ = tx.send(msg);
                });
            }
        } else {
            // TODO: Log error
        }
    }
}
