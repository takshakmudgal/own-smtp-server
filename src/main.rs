use std::{
    any::type_name,
    io::{BufRead, BufReader, BufWriter, Write},
    net::{Ipv4Addr, TcpListener, TcpStream},
};

static LOCALHOST: Ipv4Addr = Ipv4Addr::LOCALHOST;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn handle_client(stream: TcpStream) {
    println!("Client connected {}", stream.peer_addr().unwrap());

    let mut writer = BufWriter::new(&stream);
    writer
        .write_all(format!("220 {LOCALHOST} ESMTP Ready\r\n").as_bytes())
        .expect("could not write");
    writer.flush().unwrap();

    let mut reader = BufReader::new(&stream);
    let mut command: u32 = 0;

    loop {
        let mut response = String::new();
        let n = reader.read_line(&mut response).unwrap();
        if n == 0 {
            break;
        }

        println!("Loop entered {}", command);

        // HELO
        if command == 0 && response.trim().starts_with("HELO") {
            writer.write_all(b"250 OK\r\n").unwrap();
            println!("HELO {}", command);
            command += 1;
            writer.flush().unwrap();
            println!("MOVED TO {}", command);
            continue;
        } else if command == 0 {
            writer.write_all(b"500 NO\r\n").unwrap();
            writer.flush().unwrap();
            continue;
        }

        // MAIL FROM
        if command == 1 && response.trim().contains("MAIL FROM") {
            writer.write_all(b"250 OK\r\n").unwrap();
            println!("MAIL_FROM {}", command);
            command += 1;
            writer.flush().unwrap();
            println!("MOVED TO {}", command);
            continue;
        } else if command == 1 {
            writer.write_all(b"555\r\n").unwrap();
            writer.flush().unwrap();
            continue;
        }
    }
}

fn main() {
    let port = 3000;
    let listener = TcpListener::bind(format!("{LOCALHOST}:{port}")).unwrap();

    for stream in listener.incoming() {
        handle_client(stream.unwrap());
    }
}
