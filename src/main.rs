use std::sync::Arc;

use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use tokio::sync::Mutex;
use warp::{filters::{body::json, ws::Message}, Filter};
use serde::{Deserialize, Serialize};

mod config;
mod listener;
mod grid;

#[tokio::main]
async fn main() {
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            ws.on_upgrade(|websocket| async {
                // Handle WebSocket connection
                let (sender, mut receiver) = websocket.split();
                let sendfr = Arc::new(Mutex::new(sender));

                // bleh

                while let Some(msg) = receiver.next().await {
                    match msg {
                        Ok(message) => {
                            if let Ok(str) = message.to_str() {
                                let cloned = sendfr.clone();
                                
                                if let Ok(grid) = grid::execute_lua(str) {
                                    tokio::spawn(async move {
                                        let json_data = json!(grid.data);
                                        let msg = Message::text(json_data.to_string());
                                        if let Err(message_send_error) = cloned.lock().await.send(msg).await {
                                            println!("error: {}", message_send_error);
                                        }
                                    });
                                } else {
                                    // Handle the error case if needed
                                    println!("Failed to execute Lua script");
                                }
                            };
                        },
                        Err(err) => {
                            println!("Connection error: {}", err);
                        }
                    }
                }
            })
        });

    warp::serve(ws_route).run(([0, 0, 0, 0], 8080)).await;
}