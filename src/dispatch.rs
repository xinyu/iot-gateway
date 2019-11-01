use std::io::prelude::*;
use std::net::TcpStream;

pub fn handle_message(mut stream: TcpStream) {
    println!("new client!  {}", stream.peer_addr().unwrap());
    
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let msg_hello = b"msg:hello";

    let (status, contents) = if buffer.starts_with(msg_hello) {
        ("OK\r\n", "welcome\r\n")
    } else {
        ("ERROR\r\n", "")
    };

    let response = format!("{}{}", status, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
