use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn handle_client(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut buf: [u8; 255] = [0x00; 255];

    loop {
        let res = stream.read(&mut buf);
        match res {
            Ok(n) => {
                if n == 0 {
                    return Ok(());
                }
                if buf[n - 1] == 0 {
                    stream.write(&[0]).unwrap();
                }
            }
            Err(err) => {
                panic!("{}", err);
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3003")?;

    for stream in listener.incoming() {
        let _ = handle_client(stream?);
    }

    Ok(())
}
