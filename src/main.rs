use std::sync::Arc;
use futures_util::StreamExt;
use tokio::sync::Mutex;
use warp::Filter;

use backendcompiler::{
    connections::connections::{ClientAction, ClientData, ClientMessage},
    *,
};

fn process_source(source_code: &str) -> Result<Grid, ()> {
    match grid::execute_lua(source_code) {
        Ok(grid) => Ok(grid),
        Err(e) => {
            println!("Failed to execute Lua script: {}", e);
            Err(())
        }
    }
}

async fn match_request_action(
    mutex_sender: connections::connections::WebSocketSender,
    action: connections::connections::ClientAction,
    client_data: ClientData,
) {
    match action {
        ClientAction::ProcessSourceCode => {
            let source_code = &client_data.source.to_owned();
            if let Ok(grid) = process_source(source_code) {
                tokio::spawn(async move {
                    connections::connections::send_full_grid_data(mutex_sender, grid).await;
                });
            };
        }

        ClientAction::PostToBucket => {
            let source_code = &client_data.source.to_owned();
            if let Ok(grid) = process_source(source_code) {
                let mut file_extension = ".gif";
                let (path_to_image, image_uuid) = if grid.frame_count() > 1 {
                    render::image::grid_to_gif(&grid)
                } else {
                    file_extension = ".png";
                    render::image::grid_to_png(&grid)
                };

                match render::net::upload_to_minio(&path_to_image, &image_uuid, file_extension).await {
                    Ok(url_result) => {
                        tokio::spawn(async move {
                            connections::connections::send_url_to_client(mutex_sender, &url_result).await;
                        });
                    },
                    Err(e) => println!("Error when uploading to minio {}", e)
                }
            }
        }

        ClientAction::RenderPreview => {}
    }
}

async fn process_message(
    msg: warp::ws::Message,
    mutex_sender: connections::connections::WebSocketSender,
) {
    if let Ok(text) = msg.to_str() {
        println!("Received message: {}", text);

        match serde_json::from_str::<ClientMessage>(text) {
            Ok(packet) => {
                match serde_json::from_value::<ClientData>(packet.data) {
                    Ok(client_data) => {
                        match_request_action(mutex_sender, packet.action, client_data).await
                    }
                    Err(e) => println!("Error extracting packet data {}", e),
                }
            }

            Err(e) => println!("Error processing client message: {}", e),
        }
    }
}

async fn accept_websocket(websocket: warp::ws::WebSocket) {
    let (sender, mut receiver) = websocket.split();
    let mutex_sender = Arc::new(Mutex::new(sender));

    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(msg) => process_message(msg, mutex_sender.clone()).await,
            Err(err) => println!("Connection error: {}", err),
        }
    }
}

#[tokio::main]
async fn main() {
    //
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| ws.on_upgrade(accept_websocket));

    warp::serve(ws_route).run(WS_ENDPOINT).await;
}
