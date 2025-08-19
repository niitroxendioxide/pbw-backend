use std::sync::Arc;
use tokio::sync::Mutex;
use serde_json::json;
use warp::filters::ws::Message;
use futures_util::{stream::SplitSink, SinkExt};

use crate::grid::{Grid};

use serde::Serialize;

#[derive(Serialize)]
struct FrameMessage<'a> {
    frame_id: usize,
    frame_data: &'a Vec<[u8; 4]>,
}

pub type WebSocketSender = Arc<Mutex<SplitSink<warp::ws::WebSocket, Message>>>;

pub async fn send_full_grid_data(ws_sender: WebSocketSender, grid: Grid) {

    println!("There is {} frames", grid.frames.len());

    for index in 0..grid.frames.len() {
        println!("Now sending frame: {}", index);
        send_frame_to_client(ws_sender.clone(), &grid, index).await;
    }
}

fn wrap_frame_message<'a>(frame_id: usize, frame_data: &'a Vec<[u8; 4]>) -> FrameMessage<'a> {
    FrameMessage {
        frame_id,
        frame_data,
    }
}

pub async fn send_frame_to_client(ws_sender: WebSocketSender, grid: &Grid, frame: usize) {
    let frame_ref = &grid.frames[frame];
    let frame_msg = wrap_frame_message(frame_ref.id, &frame_ref.data);

    let json_data = serde_json::to_string(&frame_msg).unwrap();
    let msg = Message::text(json_data);
    if let Err(message_send_error) = ws_sender.lock().await.send(msg).await {
        println!("error: {}", message_send_error);
    }
}