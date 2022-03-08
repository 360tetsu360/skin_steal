use crate::inject::inject;
use libflate::deflate::Decoder;
use packets::{Packets, SkinHeaderPacket};
use std::io::Read;
use std::{
    collections::{BTreeMap, HashMap},
    net::{SocketAddr, UdpSocket},
};

mod inject;

fn main() {
    if !std::path::Path::new("./skins").is_dir() {
        std::fs::create_dir(std::path::Path::new("./skins")).unwrap();
    }

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

    let mut headers: HashMap<u32, SkinHeaderPacket> = HashMap::new();
    let mut skins: HashMap<u32, BTreeMap<u32, String>> = HashMap::new();
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
            Packets::SkinHeader(e) => {
                let id = e.packet_id;
                headers.insert(id, e);
                skins.insert(id, BTreeMap::new());
            }
            Packets::SkinPayload(e) => {
                if headers.contains_key(&e.packet_id) && skins.contains_key(&e.packet_id) {
                    let id = e.packet_id;
                    skins
                        .get_mut(&e.packet_id)
                        .as_mut()
                        .unwrap()
                        .insert(e.index, e.data);
                    if headers.get(&id).as_ref().unwrap().len
                        == skins.get(&id).as_ref().unwrap().len() as u32
                    {
                        let header = headers.remove(&id).unwrap();
                        let payload = skins.remove(&id).unwrap();
                        gen_skin(header, payload);
                    }
                }
            }
        }
    }
}

fn gen_skin(header: SkinHeaderPacket, payload: BTreeMap<u32, String>) {
    let mut skindata = String::new();
    for skin in payload {
        skindata += &skin.1;
    }

    let buff = base64::decode(skindata).unwrap();

    let mut decoder = Decoder::new(&buff[..]);
    let mut decoded_data = Vec::new();
    decoder.read_to_end(&mut decoded_data).unwrap();

    image::save_buffer(
        &std::path::Path::new(&format!("./skins/{}.png", header.name)),
        &decoded_data[..],
        header.width,
        header.height,
        image::ColorType::Rgba8,
    )
    .unwrap();

    println!("Stole {}'s skin", header.name);
}
