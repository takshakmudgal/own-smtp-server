use std::{
    io::Write,
    net::{Ipv4Addr, TcpListener},
};

static LOCALHOST: Ipv4Addr = Ipv4Addr::LOCALHOST;

fn main() {
    let port = 3000;
    let listener = TcpListener::bind(format!("{LOCALHOST}:{port}")).unwrap();
    for stream in listener.incoming() {
        let message = format!("220 {LOCALHOST} SMTP Ready\r\n");
        let _ = stream.unwrap().write(message.as_bytes());
    }
}
