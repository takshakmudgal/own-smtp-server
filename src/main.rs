use std::{
    io::{BufRead, BufReader, BufWriter, Write},
    net::{Ipv4Addr, TcpListener, TcpStream},
};

static LOCALHOST: Ipv4Addr = Ipv4Addr::LOCALHOST;

fn handle_client(stream: TcpStream) {
    println!("Client connected");

    let mut writer = BufWriter::new(&stream);
    writer
        .write_all(format!("220 {LOCALHOST} ESMTP Ready\r\n").as_bytes())
        .expect("could not write");
    writer.flush().expect("could not flush");

    let mut reader = BufReader::new(&stream);
    let mut response = String::new();
    reader.read_line(&mut response).expect("could not read");
    println!("Server received {}", response);
}

fn main() {
    let port = 3000;
    let listener = TcpListener::bind(format!("{LOCALHOST}:{port}")).unwrap();
    for stream in listener.incoming() {
        let stream = stream.expect("Unable to accept");
        handle_client(stream);
    }
}
