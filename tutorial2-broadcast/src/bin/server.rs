use futures::SinkExt;
use futures::StreamExt;
use std::collections::HashMap;
use std::error::Error;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::{channel, Sender};
use tokio_websockets::{Message, ServerBuilder, WebSocketStream};
use serde::{Deserialize, Serialize};

// Struct untuk parse pesan masuk dari YewChat client
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct IncomingMessage {
    message_type: String,
    data: Option<String>,
}

// Struct untuk kirim daftar users ke client
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct OutgoingUsers {
    message_type: String,
    data_array: Vec<String>,
}

// Struct untuk kirim pesan broadcast ke client
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct OutgoingMessage {
    message_type: String,
    data: String,
}

// Struct untuk isi data pesan (from + message)
#[derive(Serialize)]
struct MessageData {
    from: String,
    message: String,
}

type UserMap = Arc<Mutex<HashMap<SocketAddr, String>>>;

async fn handle_connection(
    addr: SocketAddr,
    mut ws_stream: WebSocketStream<TcpStream>,
    bcast_tx: Sender<String>,
    users: UserMap,
) {
    let mut bcast_rx = bcast_tx.subscribe();

    loop {
        tokio::select! {
            incoming = ws_stream.next() => {
                match incoming {
                    Some(Ok(msg)) => {
                        if let Some(text) = msg.as_text() {
                            log::debug!("From client {addr:?}: {text}");

                            // Parse JSON dari YewChat
                            if let Ok(parsed) = serde_json::from_str::<IncomingMessage>(text) {
                                match parsed.message_type.as_str() {

                                    // Client register dengan username
                                    "register" => {
                                        if let Some(username) = parsed.data {
                                            println!("New connection from Affandi's Computer {addr:?} as {username}");
                                            users.lock().unwrap().insert(addr, username);

                                            // Broadcast daftar user terbaru ke semua client
                                            let user_list: Vec<String> = users
                                                .lock()
                                                .unwrap()
                                                .values()
                                                .cloned()
                                                .collect();

                                            let response = OutgoingUsers {
                                                message_type: "users".to_string(),
                                                data_array: user_list,
                                            };

                                            bcast_tx
                                                .send(serde_json::to_string(&response).unwrap())
                                                .unwrap();
                                        }
                                    }

                                    // Client kirim pesan chat
                                    "message" => {
                                        if let Some(content) = parsed.data {
                                            let username = users
                                                .lock()
                                                .unwrap()
                                                .get(&addr)
                                                .cloned()
                                                .unwrap_or_else(|| addr.to_string());

                                            let msg_data = MessageData {
                                                from: username.clone(),
                                                message: content.clone(),
                                            };

                                            let response = OutgoingMessage {
                                                message_type: "message".to_string(),
                                                data: serde_json::to_string(&msg_data).unwrap(),
                                            };

                                            println!("From {username}: {content}");
                                            bcast_tx
                                                .send(serde_json::to_string(&response).unwrap())
                                                .unwrap();
                                        }
                                    }

                                    _ => {
                                        println!("Unknown message type from {addr:?}");
                                    }
                                }
                            }
                        }
                    }
                    _ => {
                        println!("Client {addr:?} disconnected");

                        // Hapus user dari map dan broadcast ulang daftar user
                        users.lock().unwrap().remove(&addr);

                        let user_list: Vec<String> = users
                            .lock()
                            .unwrap()
                            .values()
                            .cloned()
                            .collect();

                        let response = OutgoingUsers {
                            message_type: "users".to_string(),
                            data_array: user_list,
                        };

                        let _ = bcast_tx.send(serde_json::to_string(&response).unwrap());
                        break;
                    }
                }
            }
            msg = bcast_rx.recv() => {
                if let Ok(msg) = msg {
                    ws_stream.send(Message::text(msg)).await.unwrap();
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let (bcast_tx, _) = channel(16);
    let users: UserMap = Arc::new(Mutex::new(HashMap::new()));
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Listening on port 8080");

    loop {
        let (socket, addr) = listener.accept().await?;
        let bcast_tx = bcast_tx.clone();
        let users = users.clone();
        tokio::spawn(async move {
            let ws_stream = ServerBuilder::new().accept(socket).await.unwrap();
            handle_connection(addr, ws_stream, bcast_tx, users).await;
        });
    }
}