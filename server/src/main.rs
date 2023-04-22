use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str;

#[derive(Serialize, Deserialize)]
struct ClientData {
    client_id: u8,
    movement: String,
}

// used for any 3D value (position, velocity, acceleration)
#[derive(Serialize, Deserialize)]
struct Coords {
    x: f32,         // vec3() is f32, not f64
    y: f32,
    z: f32,
}

fn handle_client(mut stream: TcpStream) {
    let mut client_buf = [0 as u8; 50];     // using 50 byte buf
    let mut coords = Coords {x:0.0, y:0.0, z:0.0};

    while match stream.read(&mut client_buf) {
        Ok(size) => {
            // process client messages
            let message : &str = str::from_utf8(&client_buf[0..size]).unwrap();
            if message.len() > 0 {
                let value : ClientData = serde_json::from_str(message).unwrap();

                // process keyboard input, update the new position of cube
                if value.movement == "down" {
                    coords.z += -0.1;
                } else if value.movement == "up" {
                    coords.z += 0.1;
                } else if value.movement == "left" {
                    coords.x += -0.1;
                } else if value.movement == "right" {
                    coords.x += 0.1;
                }

                // send back serialized coords to the client
                let coords_str = serde_json::to_string(&coords).unwrap();
                stream.write(coords_str.as_bytes()).unwrap();

                // debugging
                println!("received movement: {}", value.movement);
                println!("sending coords: {}, {}, {}", coords.x, coords.y, coords.z);
            }

            // status boolean
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
