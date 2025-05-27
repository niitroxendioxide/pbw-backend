use std::error;
use tokio_tungstenite::tungstenite::{handshake::server::{Request, Response}, Message};
use futures_util::{StreamExt};
 
mod config;
mod grid;
mod listener;

use crate::grid::{execute_lua, Grid};

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn error::Error>> {
    let port = config::statics::PORT;

    let new_listener = listener::Listener::new().await;
    if let Err(e) = new_listener.connect(port, handle).await {
        println!("Error connecting port: {}", e);
    } else {
        println!("Opened port: {}", port);
    }

    Ok(())
}

async fn process(stream: tokio::net::TcpStream) -> Result<(), tokio_tungstenite::tungstenite::Error> {
    let callback = |_req: &Request, response: Response| {
        Ok(response)
    };

    let ws_stream = tokio_tungstenite::accept_hdr_async(stream, callback).await?;
    let (mut _transmitter, mut receiver) = ws_stream.split();

    println!("transmitter Bv");

    tokio::spawn(async move {
        while let Some(message) = receiver.next().await {
            match message {
                Ok(msg) => {
                    if let Message::Text(text) = msg {
                        println!("{}", text);
                        let result: Result<Grid, mlua::Error> = execute_lua(&text);

                        match result {
                            Ok(grid) => {
                                for pixel in grid.data {
                                    println!("RGB({}, {}, {})", pixel[0], pixel[1], pixel[2])
                                }
                            }

                            Err(e) => {
                                println!("Code did not return a grid! {}", e);
                            }
                        }
                        //println!("{}", result);
                    }

                },
                Err(e) => println!("{}", e),
            }
        }
    });

    Ok(())
}

async fn handle(stream: tokio::net::TcpStream) -> () {
    match process(stream).await {
        Ok(_) => println!("Connection handled successfully"),
        Err(e) => eprintln!("Connection error: {}", e),
    }
}