use std::io;

use futures::{Sink, SinkExt, StreamExt};
use tokio_tungstenite::connect_async;
use tungstenite::Message;

#[tokio::main]
async fn main() {
    let url = "ws://localhost:3113/websocket";

    let (mut socket, _) = connect_async(url)
        .await
        .expect("Couldn't connect to websocket");

    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Cant read input");
        let input = input.trim();

        socket
            .send(Message::Text(input.into()))
            .await
            .expect("Cant send message");

        if let Some(msg) = socket.next().await {
            match msg {
                Ok(Message::Text(text)) => println!("Recieved: {text}"),
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error: {e}")
                }
            }
        }
    }
}
