use std::net::TcpStream;
use std::{process, str};
use std::io::{Read,Write,ErrorKind};

pub fn read_data(stream: &mut TcpStream) -> String {
    let mut size_buf = [0 as u8; 4];
    let size:u32;
    match stream.peek(&mut size_buf) {
        Ok(4) => {
            size = u32::from_be_bytes(size_buf);
        },
        Ok(_) => return "".to_string(),
        Err(ref e) if e.kind() == ErrorKind::WouldBlock => return "".to_string(),
        Err(_) => {
            eprintln!("server disconnected");
            process::exit(1);
        }
    }

    let s_size: usize = size.try_into().unwrap();
    let mut read_buf = vec![0 as u8; s_size];
    let mut message : &str = "";
    match stream.peek(&mut read_buf) {
        Ok(bytes_read) if bytes_read == s_size => {
            stream.read_exact(&mut read_buf).expect("read_exact did not read the same amount of bytes as peek");
            message = str::from_utf8(&read_buf[4..]).expect("Error converting buffer to string");
        },
        Ok(_) => {},
        Err(ref e) if e.kind() == ErrorKind::WouldBlock => (),
        Err(_) => {
            eprintln!("server disconnected");
            process::exit(1);
        }
    }
    return message.to_string();
}

pub fn write_data(stream: &mut TcpStream, string: String) {
    let size = string.len() as u32 + 4;
    let message = [u32::to_be_bytes(size).to_vec(), string.clone().into_bytes()].concat();
    match stream.write(&message) {
        Ok(_) => (),
        Err(ref e) if e.kind() == ErrorKind::WouldBlock => (),
        Err(_) => (),
    }
}

// Takes a string and returns a vector of bytes with a 4-byte size field prepended.
pub fn to_network_bytes(string: &str) -> Vec<u8> {
    let send_size = string.len() as u32 + 4;
    return [u32::to_be_bytes(send_size).to_vec(), string.as_bytes().to_vec()].concat();
}

pub fn poll_size(stream: &TcpStream) -> i32 {
    let mut size_buf = [0 as u8; 4];
    match stream.peek(&mut size_buf) {
        Ok(4) => {
            // big-endian for networks. it's tradition, dammit!
            return i32::from_be_bytes(size_buf);
        },
        Ok(_) => {
            // incomplete size field, wait for next tick
            return 0;
        },
        Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
            return 0;
        }
        Err(e) => {
            eprintln!("Failed to read message size from server: {}",e);
            // TODO: handle lost client
            return -1;
        }
    }
}