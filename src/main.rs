use std::net::{Ipv4Addr, TcpListener};

static LOCALHOST: Ipv4Addr = Ipv4Addr::LOCALHOST;

fn main() {
    let port = 25;
    let result = TcpListener::bind(format!("{LOCALHOST}:{port}")).is_ok();
    dbg!(result);
}
