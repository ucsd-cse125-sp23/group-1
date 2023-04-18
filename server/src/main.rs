use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str;

#[derive(Serialize, Deserialize)]
struct ClientData {
    client_id: u8,
    movement: String,
}

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50]; // using 50 byte buff
    while match stream.read(&mut data) {
        Ok(size) => {
            stream.write(&data[0..size]).unwrap();
            let message : &str = str::from_utf8(&data[0..size]).unwrap();
            if message.len() > 0 {
                let value : ClientData = serde_json::from_str(message).unwrap();
                println!("received: {}", value.movement);
            }
            size > 0
        },
        Err(_) => {
            println!("An error occurred");
            false
        }
    } {}
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("localhost:8080")?;

    // accepts connections automatically
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}
