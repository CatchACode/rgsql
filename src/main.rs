use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn handle_client(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut buf: [u8; 255] = [0; 255];

    while stream.read(&mut buf)? > 0 {
        stream.write(&buf)?;
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3003")?;

    for stream in listener.incoming() {
        let _ = handle_client(stream?);
    }

    Ok(())
}
