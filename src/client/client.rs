use std::net::{Shutdown, TcpStream};
use std::io::{Read, Write};

fn main() -> std::io::Result<()> {
    for _ in 0..5 {
        let mut stream = TcpStream::connect("localhost:8080")?;
        stream.write(&[69])?;
        stream.read(&mut [0; 128])?;
        // stream.shutdown(Shutdown::Both);
    }
    Ok(())
}