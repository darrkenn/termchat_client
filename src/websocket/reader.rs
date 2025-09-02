use futures::{StreamExt, stream::SplitStream};
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use tungstenite::Message;

use crate::Messages;

pub async fn websocket_reader(
    messages: Messages,
    mut ws_r: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
) {
    while let Some(msg) = ws_r.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                let mut msgs = messages.lock().unwrap();
                msgs.push(text.to_string());
            }
            Ok(Message::Close(reason)) => {
                break;
            }
            Ok(_) => {}
            Err(e) => {
                break;
            }
        }
    }
}
