use std::sync::Arc;
use tokio::sync::Mutex;
use warp::filters::ws::Message;
use futures_util::{stream::SplitSink, SinkExt};
use serde::{Serialize, Deserialize};

use crate::grid::{Grid};


#[derive(Serialize)]
struct FrameMessage<'a> {
    frame_id: usize,
    frame_data: &'a Vec<[u8; 4]>,
}

#[derive(Serialize)]
struct ServerResponseData<'a> {
    frame: FrameMessage<'a>,
}

#[derive(Debug, Deserialize, Clone)]
pub enum ClientAction {
    ProcessSourceCode,
    PostToBucket,
    RenderPreview,
}

#[derive(Debug, Serialize)]
pub enum ServerAction {
    FrameData,
    Error,
    UploadSuccess,
    PreviewReady,
}

#[derive(Debug, Deserialize)]
pub struct ClientData {
    pub source: String,
}


#[derive(Debug, Deserialize, Clone)]
pub struct ClientMessage {
    pub action: ClientAction,
    pub data: serde_json::Value,
    pub request_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ServerMessage {
    pub action: ServerAction,
    pub data: serde_json::Value,
}

pub type WebSocketSender = Arc<Mutex<SplitSink<warp::ws::WebSocket, Message>>>;

pub async fn send_full_grid_data(ws_sender: WebSocketSender, grid: Grid) {
    for index in 0..grid.frames.len() {
        send_frame_to_client(ws_sender.clone(), &grid, index).await;
    }
}


fn wrap_frame_message<'a>(frame_id: usize, frame_data: &'a Vec<[u8; 4]>) -> ServerMessage {
    /*let frame_data_packed = FrameMessage {
        frame_data,
        frame_id,
    };*/

    ServerMessage {
        action: ServerAction::FrameData,
        data: serde_json::json!({
            "frame": {
                "frame_data": frame_data,
                "frame_id": frame_id,
            },
        }),
    }
}

pub async fn send_url_to_client(ws_sender: WebSocketSender, minio_url: &str) {
    let packet = ServerMessage {
        action: ServerAction::UploadSuccess,
        data: serde_json::json!({
            "urlBucket": minio_url,
        })
    };

    let stringified = serde_json::to_string(&packet).unwrap();
    let sent_packet = Message::text(stringified);

    if let Err(message_sent_error) = ws_sender.lock().await.send(sent_packet).await {
        println!("Error upon sending url to client: {}", message_sent_error);
    }

}

pub async fn send_frame_to_client(ws_sender: WebSocketSender, grid: &Grid, frame: usize) {
    let frame_ref = &grid.frames[frame];
    let frame_msg = wrap_frame_message(frame_ref.id, &frame_ref.data);

    let json_data = serde_json::to_string(&frame_msg).unwrap();
    let msg = Message::text(json_data);
    if let Err(message_send_error) = ws_sender.lock().await.send(msg).await {
        println!("Error sending frame data to client: {}", message_send_error);
    }
}