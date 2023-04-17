use serde::{Deserialize, Serialize};
use std::io;
use std::io::{Read,Write};
use std::net::{TcpStream};

#[derive(Serialize, Deserialize)]
struct ClientData {
    client_id: u8,
    movement: String,
}

fn main() -> std::io::Result<()> {
    while True {
        let mut stream = TcpStream::connect("localhost:8080")?;
        // if Ok(stream) {
        //     println!("Connected to the server!");
        // } else {
        //     println!("Couldn't connect to server...");
        // }

        println!("Choose your movement [forward, backward, left, right]: ");

        let mut input = String::new();
        let n = io::stdin().read_line(&mut input).unwrap();
        if n > 0 {
            let client_data = ClientData {
                client_id: 1,
                movement: input,
            };
            let j = serde_json::to_string(&client_data)?;
            stream.write(j.as_bytes())?;
            stream.read(&mut [0; 128])?;
        }
    }
    Ok(())
}