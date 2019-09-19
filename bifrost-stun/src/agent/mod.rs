use crate::message::{Message, TransactionId};
use std::collections::HashMap;
use std::future::Future;
use std::io;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio_sync::{oneshot, Mutex};
use tokio_timer::Timeout;

type TransactionKey = (TransactionId, SocketAddr);
type TransactionMap = HashMap<TransactionKey, oneshot::Sender<Message>>;

#[derive(Clone)]
pub struct Agent<F> {
    on_send: F,
    transactions: Arc<Mutex<TransactionMap>>,
}

impl<F, Fut> Agent<F>
where
    F: Fn(Message, SocketAddr) -> Fut,
    Fut: Future<Output = io::Result<()>>,
{
    pub fn new(on_send: F) -> Self {
        Self {
            on_send,
            transactions: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn send(&self, msg: Message, addr: SocketAddr) -> io::Result<Message> {
        let (tx, rx) = oneshot::channel();

        // TODO: Handle key conflicts.
        self.transactions
            .lock()
            .await
            .insert((msg.transaction_id, addr), tx);

        // Let the callback actually send out the message.
        (self.on_send)(msg, addr).await?;

        // TODO: Clean up transaction on timeout.
        let res = Timeout::new(rx, Duration::from_secs(3))
            .await
            .map_err(|_| io::Error::from(io::ErrorKind::TimedOut))?;

        res.map_err(|_| io::Error::from(io::ErrorKind::Other))
    }

    pub async fn on_recv(&self, msg: Message, addr: SocketAddr) {
        let tx = self
            .transactions
            .lock()
            .await
            .remove(&(msg.transaction_id, addr));

        if let Some(tx) = tx {
            let _ = tx.send(msg);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util;
    use futures_util::future;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn basic() {
        tokio_test::block_on(async {
            let agent = Agent::new(|_, _| future::ok(()));
            let done = Arc::new(AtomicUsize::new(0));

            let addrs = test_util::get_test_addrs();
            let len = addrs.len();
            for addr in addrs {
                let msg = test_util::new_test_msg(addr);

                let a = agent.clone();
                let d = Arc::clone(&done);
                tokio_executor::spawn(async move {
                    if a.send(msg, addr).await.is_ok() {
                        d.fetch_add(1, Ordering::SeqCst);
                    }
                });

                let a = agent.clone();
                tokio_executor::spawn(async move {
                    // Simulate network latency.
                    tokio_timer::delay_for(Duration::from_millis(500)).await;
                    a.on_recv(test_util::new_test_msg(addr), addr).await;
                });
            }

            // Wait for all tasks to finish.
            tokio_timer::delay_for(Duration::from_secs(1)).await;

            assert_eq!(done.load(Ordering::SeqCst), len);
        });
    }
}
