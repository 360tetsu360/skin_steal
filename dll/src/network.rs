use std::{
    net::{SocketAddr, UdpSocket},
    sync::Arc,
};

use packets::SkinPayloadPacket;

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

static mut PACKETID: u32 = 0;

pub unsafe fn skin(runtime: u64, name: String, width: u32, height: u32, data: &str) {
    if SOCK.is_some() {
        let remote_addr: SocketAddr = "127.0.0.1:19120".parse().unwrap();
        let len = (data.len() - data.len() % 1000) / 1000 + 1;

        let header = packets::Packets::SkinHeader(packets::SkinHeaderPacket {
            runtime_id: runtime,
            name,
            width,
            height,
            packet_id: PACKETID,
            len: len as u32,
        });
        SOCK.as_ref()
            .unwrap()
            .send_to(&header.encode(), remote_addr)
            .unwrap();

        let str_len = data.len();
        let mut payloads = vec![];
        let mut pos = 0;
        for i in 0..len {
            if str_len - pos > 1000 {
                payloads.push(packets::Packets::SkinPayload(SkinPayloadPacket {
                    packet_id: PACKETID,
                    index: i as u32,
                    data: data[pos..pos + 1000].to_string(),
                }));
                pos += 1000;
            } else {
                payloads.push(packets::Packets::SkinPayload(SkinPayloadPacket {
                    packet_id: PACKETID,
                    index: i as u32,
                    data: data[pos..str_len].to_string(),
                }));
            }
        }

        for payload in payloads {
            SOCK.as_ref()
                .unwrap()
                .send_to(&payload.encode(), remote_addr)
                .unwrap();
        }
    }
}
