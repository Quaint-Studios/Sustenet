use std::{ fmt::Debug, io::Error, net::SocketAddr };

use crate::{ lselect, utils::constants::DEFAULT_IP };

use tokio::{ io, net::{ TcpListener, TcpStream }, sync::broadcast };

pub(crate) trait Server: Log {
    async fn create_listener(port: u16) -> io::Result<TcpListener> {
        let addr = format!("{}:{}", DEFAULT_IP, port);
        let listener = match TcpListener::bind(&addr).await {
            Ok(l) => {
                Self::success(&format!("Master server started on {addr}")).await;
                l
            }
            Err(e) => {
                Self::error(&format!("Failed to bind to {addr}")).await;
                return Err(Error::new(e.kind(), format!("Failed to bind to ({addr}): {e}")));
            }
        };
        Ok(listener)
    }

    async fn listen<EventType: Clone + Debug>(
		event_tx: &broadcast::Sender<EventType>,
        event_rx: &mut broadcast::Receiver<EventType>,
        listener: TcpListener
    ) -> io::Result<()> {
        lselect!(
            event = event_rx.recv() => {
                if let Ok(event) = event {
                    Self::handle_events::<EventType>(event).await?;
                }
            }
            res = listener.accept() => Self::handle_listener(res, event_tx).await?

        )
    }

    async fn handle_events<EventType: Clone + Debug>(event: EventType) -> io::Result<()>;

    async fn handle_listener<EventType: Clone + Debug>(
        res: io::Result<(TcpStream, SocketAddr)>,
        event_tx: &broadcast::Sender<EventType>
    ) -> io::Result<()> {
        let (stream, peer) = match res {
            Ok(pair) => pair,
            Err(e) => {
                Self::error(&format!("Failed to accept connection: {e}")).await;
                return Err(Error::new(e.kind(), format!("Failed to accept connection: {e}")));
            }
        };

        Self::add_connection(stream, event_tx.clone(), peer).await?;

        Self::debug(&format!("Accepted connection from {peer}")).await;

        Ok(())
    }

    async fn add_connection<EventType: Clone + Debug>(
        stream: TcpStream,
        event_tx: broadcast::Sender<EventType>,
        peer: SocketAddr
    ) -> io::Result<()>;
}

pub(crate) trait Log {
    async fn debug(message: &str);
    async fn info(message: &str);
    async fn warning(message: &str);
    async fn error(message: &str);
    async fn success(message: &str);
}
