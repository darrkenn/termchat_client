use futures::{SinkExt, stream::SplitSink};
use tokio::{net::TcpStream, sync::mpsc};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use tungstenite::Message;

pub async fn websocket_writer(
    mut ws_w: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    mut rx: mpsc::Receiver<Message>,
) {
    while let Some(msg) = rx.recv().await {
        if let Err(e) = ws_w.send(msg).await {
            eprintln!("{e}");
            break;
        }
    }
}
