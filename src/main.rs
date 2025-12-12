use std::net::{Ipv4Addr, TcpListener};

static LOCALHOST: Ipv4Addr = Ipv4Addr::LOCALHOST;

fn main() {
    let port = 25;
    let listner = TcpListener::bind(format!("{LOCALHOST}:{port}")).unwrap();
    match listner.accept() {
        Ok((_socket, addr)) => println!("new client: {addr:?}"),
        Err(e) => println!("couldn't get client: {e:?}"),
    }
}
