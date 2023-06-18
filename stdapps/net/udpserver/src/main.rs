extern crate alloc;

use std::str::FromStr;

use std::io;
use std::net::{IpAddr, UdpSocket};

const LOCAL_IP: &str = "10.0.2.15";
const LOCAL_PORT: u16 = 5555;

fn receive_loop() -> io::Result<()> {
    let (addr, port) = (IpAddr::from_str(LOCAL_IP).unwrap(), LOCAL_PORT);
    let socket = UdpSocket::bind::<(IpAddr, u16)>((addr, port).into())?;
    println!("listen on: {}", socket.local_addr().unwrap());
    let mut buf = [0u8; 1024];
    loop {
        match socket.recv_from(&mut buf) {
            Ok((size, addr)) => {
                println!("recv: {}Bytes from {}", size, addr);
                let mid = core::str::from_utf8(&buf).unwrap();
                println!("{}", mid);
                let mid = ["response_", mid].join("");
                socket.send_to(mid.as_bytes(), addr)?;
                buf = [0u8; 1024];
            }
            Err(e) => return Err(e),
        };
    }
}

fn main() {
    println!("Hello, simple udp client!");
    receive_loop().expect("test udp client failed");
}
