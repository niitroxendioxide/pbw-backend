use std::{sync::atomic::{AtomicUsize, Ordering}};

use::tokio::{net::{TcpListener, TcpStream}, io};

static LISTENER_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone)]
pub struct Listener {
    pub id: usize,
}

impl Listener {
    pub async fn new() -> Self {
        let current_id = LISTENER_COUNTER.fetch_add(1, Ordering::Relaxed);
        Listener { id: current_id }
    }

    pub async fn connect<C, Fut>(&self, port: &str, callback: C) -> io::Result<()>
    where
        C: Fn(TcpStream) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let listener = TcpListener::bind(port).await?;
        println!("Listener: {} connected", self.id);

        loop {
            let (socket, _) = listener.accept().await?;
            // Spawn each connection as a separate task
            tokio::spawn(callback(socket));
        }
    }
}