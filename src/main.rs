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
    println!("client connected {}", stream.peer_addr().unwrap());

    let mut writer = BufWriter::new(&stream);
    writer
        .write_all(format!("220 {LOCALHOST} ESMTP Ready\r\n").as_bytes())
        .unwrap();
    writer.flush().unwrap();

    let mut reader = BufReader::new(&stream);
    let mut command: u32 = 0;

    loop {
        let mut response = String::new();
        let n = reader.read_line(&mut response).unwrap();
        if n == 0 {
            println!("client disconnected");
            break;
        }

        let line = response.trim();
        println!("state={} recv='{}'", command, line);

        if line.starts_with("NOOP") {
            writer.write_all(b"250 OK\r\n").unwrap();
            writer.flush().unwrap();
            continue;
        }

        if line.starts_with("RSET") {
            command = 0;
            writer.write_all(b"250 OK\r\n").unwrap();
            writer.flush().unwrap();
            println!("state reset");
            continue;
        }

        if line.starts_with("QUIT") {
            writer.write_all(b"221 OK\r\n").unwrap();
            writer.flush().unwrap();
            println!("session closed");
            break;
        }

        if command == 0 {
            if line.starts_with("HELO") {
                writer.write_all(b"250 OK\r\n").unwrap();
                writer.flush().unwrap();
                command = 1;
                println!("transition HELO -> MAIL");
            } else {
                writer.write_all(b"500 Expected HELO\r\n").unwrap();
                writer.flush().unwrap();
            }
            continue;
        }

        if command == 1 {
            if line.starts_with("MAIL FROM") {
                writer.write_all(b"250 OK\r\n").unwrap();
                writer.flush().unwrap();
                command = 2;
                println!("transition MAIL -> RCPT");
            } else {
                writer.write_all(b"503 Expected MAIL FROM\r\n").unwrap();
                writer.flush().unwrap();
            }
            continue;
        }

        if command == 2 {
            if line.starts_with("RCPT TO") {
                writer.write_all(b"250 OK\r\n").unwrap();
                writer.flush().unwrap();
                command = 3;
                println!("transition RCPT -> DATA");
            } else {
                writer.write_all(b"503 Expected RCPT TO\r\n").unwrap();
                writer.flush().unwrap();
            }
            continue;
        }

        if command == 3 {
            if line == "DATA" {
                writer.write_all(b"354 End with <CRLF>.<CRLF>\r\n").unwrap();
                writer.flush().unwrap();
                command = 4;
                println!("enter DATA mode");
            } else {
                writer.write_all(b"503 Expected DATA\r\n").unwrap();
                writer.flush().unwrap();
            }
            continue;
        }

        if command == 4 {
            if line == "." {
                writer.write_all(b"250 OK Message accepted\r\n").unwrap();
                writer.flush().unwrap();
                println!("message complete");
                command = 0;
            } else {
                println!("data: {}", line);
            }
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
