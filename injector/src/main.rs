use crate::inject::inject;
use packets::Packets;
use std::net::{SocketAddr, UdpSocket};

mod inject;

fn main() {
    let loacl_addr: SocketAddr = "127.0.0.1:19120".parse().unwrap();
    let udp = UdpSocket::bind(loacl_addr).unwrap();
    unsafe {
        let success = inject(
            "Minecraft.Windows.exe",
            &format!(
                "{}{}",
                std::env::current_dir().unwrap().to_str().unwrap(),
                "\\dll.dll"
            ),
        );
        if !success {
            panic!()
        }
    }

    loop {
        let mut buf = [0u8; 2048];
        let (size, _source) = udp.recv_from(&mut buf).unwrap();

        let data = match Packets::decode(&buf[..size]) {
            Some(p) => p,
            None => continue,
        };

        match data {
            Packets::Hello(p) => {
                println!("Connected to minecraft {}", p.address);
            }
            Packets::Log(log) => {
                println!("{}", log.msg);
            }
            Packets::EntityList(e) => {
                dbg!(e.enttend);
            }
        }
    }
}
