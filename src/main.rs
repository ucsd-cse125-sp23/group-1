// Server Code
// TODO: move to server folder? idfk how cargo works yet

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
// use serde_json::json;

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50]; // using 50 byte buff
    while match stream.read(&mut data) {
        Ok(size) => {
            stream.write(&data[0..size]).unwrap();
            println!("received `{:?}`", size);
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
