use std::{
    net::{SocketAddr, UdpSocket},
    sync::Arc,
};

static mut SOCK: Option<Arc<UdpSocket>> = None;

pub unsafe fn init() {
    let local_addr: SocketAddr = "127.0.0.1:0".parse().unwrap();
    SOCK = Some(Arc::new(UdpSocket::bind(local_addr).unwrap()));

    let remote_addr: SocketAddr = "127.0.0.1:19120".parse().unwrap();
    let data = packets::Packets::Hello(packets::HelloPacket {
        address: local_addr.to_string(),
    });
    SOCK.as_ref()
        .unwrap()
        .send_to(&data.encode(), remote_addr)
        .unwrap();
}

pub unsafe fn send(data: &str) {
    if SOCK.is_some() {
        let remote_addr: SocketAddr = "127.0.0.1:19120".parse().unwrap();
        let data = packets::Packets::Log(packets::LogPacket {
            msg: data.to_owned(),
        });
        SOCK.as_ref()
            .unwrap()
            .send_to(&data.encode(), remote_addr)
            .unwrap();
    }
}

pub unsafe fn _player_list(start: u64, end: u64) {
    if SOCK.is_some() {
        let remote_addr: SocketAddr = "127.0.0.1:19120".parse().unwrap();
        let data = packets::Packets::EntityList(packets::EntityListPacket {
            enttstart: start,
            enttend: end,
        });
        SOCK.as_ref()
            .unwrap()
            .send_to(&data.encode(), remote_addr)
            .unwrap();
    }
}
