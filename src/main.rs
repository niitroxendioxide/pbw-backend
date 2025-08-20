use std::sync::Arc;

use futures_util::{StreamExt};
use tokio::sync::Mutex;
use warp::Filter;

mod config;
mod grid;
mod connections;
mod render;

fn process_source_code(ws_sender: connections::connections::WebSocketSender, source_code: &str) {       
    if let Ok(grid) = grid::execute_lua(source_code) {
        let grid_clone = grid.clone();


        let image_id = render::image::grid_to_png(&grid_clone);

        tokio::spawn(async move {
            println!("Attempting to send grid data to client.");

            connections::connections::send_full_grid_data(ws_sender, grid_clone).await;
        });
    } else {
        println!("Failed to execute Lua script");
    }
} 

async fn accept_websocket(websocket: warp::ws::WebSocket) {
    let (sender, mut receiver) = websocket.split();
    let mutex_sender = Arc::new(Mutex::new(sender));

    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(message) => {
                if let Ok(source_code) = message.to_str() {
                    process_source_code(mutex_sender.clone(), source_code);
                };
            },
            Err(err) => {
                println!("Connection error: {}", err);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // TESTING

    //
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            ws.on_upgrade(accept_websocket)
        });

    warp::serve(ws_route).run(([0, 0, 0, 0], 8080)).await;
}