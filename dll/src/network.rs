use std::{
    io::Write,
    net::{SocketAddr, TcpStream},
    sync::{Arc, Mutex},
};

use lazy_static::lazy_static;

lazy_static! {
    static ref SOCK: Arc<Mutex<TcpStream>> = {
        let remote: SocketAddr = "127.0.0.1:19120".parse().unwrap();
        let socket = TcpStream::connect(remote).unwrap();
        Arc::new(Mutex::new(socket))
    };
}

pub unsafe fn init() {
    let data = packets::Packets::Hello(packets::HelloPacket {
        address: SOCK.lock().unwrap().local_addr().unwrap().to_string(),
    });
    send_buf(data.encode());
}

pub unsafe fn send(data: &str) {
    let data = packets::Packets::Log(packets::LogPacket {
        msg: data.to_owned(),
    });
    send_buf(data.encode());
}

pub unsafe fn skin(runtime: u64, name: String, width: u32, height: u32, data: String) {
    let data = packets::Packets::Skin(packets::SkinPacket {
        runtime_id: runtime,
        name,
        width,
        height,
        data,
    });
    send_buf(data.encode());
}

fn send_buf(mut data: Vec<u8>) {
    let len = data.len() as u16;
    let bytes = len.to_be_bytes();
    data.insert(0, bytes[0]);
    data.insert(1, bytes[1]);
    SOCK.lock().unwrap().write_all(&data[..]).unwrap();
}
