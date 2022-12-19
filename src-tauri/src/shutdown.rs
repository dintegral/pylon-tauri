use tokio::sync::{broadcast, mpsc};

pub struct Shutdown {
    shutdown: bool,
    shutdown_rx: broadcast::Receiver<()>,
    shutdown_complete_tx: mpsc::Sender<()>,
}

impl Shutdown {
    pub fn new(
        shutdown_rx: broadcast::Receiver<()>,
        shutdown_complete_tx: mpsc::Sender<()>,
    ) -> Shutdown {
        Shutdown {
            shutdown: false,
            shutdown_rx,
            shutdown_complete_tx,
        }
    }

    pub async fn recv(&mut self) {
        if self.shutdown {
            return;
        }

        let _ = self.shutdown_rx.recv().await;

        self.shutdown = true;
    }

    pub async fn send_complete(self) {
        let _ = self.shutdown_complete_tx.send(()).await;
    }
}
