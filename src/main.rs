use futures_util::StreamExt;
use warp::Filter;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            ws.on_upgrade(|websocket| async {
                // Handle WebSocket connection
                let (tx, rx) = websocket.split();
                
                // bleh
            })
        });

    warp::serve(ws_route).run(([0, 0, 0, 0], 8080)).await;
}