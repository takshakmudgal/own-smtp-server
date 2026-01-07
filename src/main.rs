use std::{
    any::type_name,
    io::{BufRead, BufReader, BufWriter, Read, Write},
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

        if response.trim().starts_with("NOOP") {
            writer.write_all(b"250 OK\r\n").unwrap();
            writer.flush().unwrap();
            continue;
        }

        if response.trim().starts_with("RSET") {
            writer.write_all(b"250 OK\r\n").unwrap();
            writer.flush().unwrap();
            command = 0;
            continue;
        }

        if response.trim().starts_with("QUIT") {
            writer.write_all(b"221 OK\r\n").unwrap();
            writer.flush().unwrap();
            break;
        }

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
        if command == 1 && response.trim().starts_with("MAIL FROM") {
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

        // RCPT TO
        if command == 2 && response.trim().starts_with("RCPT TO") {
            writer.write_all(b"250 OK\r\n").unwrap();
            println!("RCPT TO {}", command);
            command += 1;
            writer.flush().unwrap();
            println!("MOVED TO {}", command);
            continue;
        } else if command == 2 {
            writer.write_all(b"555\r\n").unwrap();
            writer.flush().unwrap();
            continue;
        }

        // DATA
        if command == 3 && response.trim().eq("DATA") {
            writer.write_all(b"354 GRANTED\r\n").unwrap();
            println!("DATA {}", command);
            command += 1;
            writer.flush().unwrap();
            println!("MOVED TO {}", command);
            continue;
        } else if command == 3 {
            writer.write_all(b"500 NO\r\n").unwrap();
            writer.flush().unwrap();
            continue;
        }

        if command == 4 && response.trim().starts_with("Date:") {
            command += 1;
            println!("MOVED TO {}", command);
            continue;
        } else if command == 4 {
            writer.write_all(b"Wrong Date Format\r\n").unwrap();
            writer.flush().unwrap();
            continue;
        }

        if command == 5 && response.trim().starts_with("From:") {
            command += 1;
            println!("MOVED TO {}", command);
            continue;
        } else if command == 5 {
            writer.write_all(b"Wrong From Format\r\n").unwrap();
            writer.flush().unwrap();
            continue;
        }

        if command == 6 && response.trim().starts_with("Subject:") {
            command += 1;
            println!("MOVED TO {}", command);
            continue;
        } else if command == 6 {
            writer.write_all(b"Wrong Subject Format\r\n").unwrap();
            writer.flush().unwrap();
            continue;
        }

        if command == 7 && response.trim().starts_with("To:") {
            command += 1;
            println!("MOVED TO {}", command);
            continue;
        } else if command == 7 {
            writer.write_all(b"Wrong To Format\r\n").unwrap();
            writer.flush().unwrap();
            continue;
        }

        if command == 8 {
            command += 1;
            println!("MOVED TO {}", command);
            continue;
        } else if command == 8 {
            writer.write_all(b"Wrong MESSAGE Format\r\n").unwrap();
            writer.flush().unwrap();
            continue;
        }

        if command == 9 && response.trim().eq(".") {
            command += 1;
            println!("MOVED TO {}", command);
            break;
        } else if command == 9 {
            writer.write_all(b"Wrong DATA Format\r\n").unwrap();
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
