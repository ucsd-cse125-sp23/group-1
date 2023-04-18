use serde::{Deserialize, Serialize};
use std::io;
use std::io::{Read,Write};
use std::net::{TcpStream};
use std::str;

#[derive(Serialize, Deserialize)]
struct ClientData {
    client_id: u8,
    movement: String,
}

struct GameState {
    placeholder: String
}

fn main() -> std::io::Result<()> {
    loop {
        let mut stream = TcpStream::connect("localhost:8080")?;
        // if Ok(stream) {
        //     println!("Connected to the server!");
        // } else {
        //     println!("Couldn't connect to server...");
        // }
        
        // 1. 4 movements
        // 2. vector rotation (3 params)
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

            let mut buf = [0 as u8; 128];
            let size = stream.read(&mut buf)?;
            let message : &str = str::from_utf8(&buf[0..size]).unwrap();
            println!("{}", message);
        }

        /*  TODO:
                1. Check for* updated state
                2. Update local game state
                3. Render world
        */ 
    }
    // Ok(())
}