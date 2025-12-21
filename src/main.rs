use std::{
    any::type_name,
    io::{BufRead, BufReader, BufWriter, Write},
    net::{Ipv4Addr, TcpListener, TcpStream},
};

static LOCALHOST: Ipv4Addr = Ipv4Addr::LOCALHOST;

// checks type of a variable
fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn handle_client(stream: TcpStream) {
    println!("Client connected");

    let mut writer = BufWriter::new(&stream);
    writer
        .write_all(format!("220 {LOCALHOST} ESMTP Ready\r\n").as_bytes())
        .expect("could not write");

    let mut reader = BufReader::new(&stream);
    let mut response = String::new();
    reader.read_line(&mut response).unwrap();

    // HELO command
    if response.trim() == "HELO" {
        writer.write_all(b"250 OK\r\n").unwrap();
    } else {
        writer.write_all(b"500 NO\r\n").unwrap();
    }

    // MAIL FROM

    writer.flush().expect("could not flush");
}

fn main() {
    let port = 3000;
    let listener = TcpListener::bind(format!("{LOCALHOST}:{port}")).unwrap();

    for stream in listener.incoming() {
        let stream = stream.expect("Unable to accept");
        handle_client(stream);
    }
}
