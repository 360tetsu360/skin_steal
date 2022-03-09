use crate::inject::inject;
use libflate::deflate::Decoder;
use packets::{Packets, SkinPacket};
use std::io::{Cursor, Read};
use std::net::{SocketAddr, TcpListener, TcpStream};

mod inject;

fn main() {
    println!(
        "{}",
        r"  
    _____________  __.___ _______      _______________________________   _____  .____     
    /   _____/    |/ _|   |\      \    /   _____/\__    ___/\_   _____/  /  _  \ |    |    
    \_____  \|      < |   |/   |   \   \_____  \   |    |    |    __)_  /  /_\  \|    |    
    /        \    |  \|   /    |    \  /        \  |    |    |        \/    |    \    |___ 
   /_______  /____|__ \___\____|__  / /_______  /  |____|   /_______  /\____|__  /_______ \
           \/        \/           \/          \/                    \/         \/        \/"
    );

    if !std::path::Path::new("./skins").is_dir() {
        std::fs::create_dir(std::path::Path::new("./skins")).unwrap();
    }

    let local_addr: SocketAddr = "127.0.0.1:19120".parse().unwrap();
    let tcp = TcpListener::bind(local_addr).unwrap();

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

    for stream in tcp.incoming().flatten() {
        std::thread::spawn(move || handle(stream));
    }
}

fn handle(mut stream: TcpStream) {
    let mut queue: (usize, Vec<u8>) = (0, vec![]);
    loop {
        let mut buf = [0u8; 2048];
        let size = stream.read(&mut buf).expect("Minecraft closed");

        if queue.0 != 0 {
            if queue.0 > size {
                let mut extra = buf[..size].to_vec();
                queue.1.append(&mut extra);
                queue.0 -= size;
            } else {
                let mut extra = buf[..queue.0].to_vec();
                queue.1.append(&mut extra);
                handle_packet(&queue.1[..]);
                queue = (0, vec![]);
            }
        }

        let mut cursor = Cursor::new(&buf[..size]);
        while size > cursor.position() as usize {
            let mut len_bytes = [0u8; 2];
            cursor.read_exact(&mut len_bytes).unwrap();
            let len = u16::from_be_bytes(len_bytes) as usize;

            if cursor.position() as usize + len > size {
                let length = size - cursor.position() as usize;
                let mut buff = vec![0u8; length];
                cursor.read_exact(&mut buff).unwrap();
                queue = (len - length, buff);
                break;
            }

            handle_packet(&buf[cursor.position() as usize..cursor.position() as usize + len]);
            cursor.set_position(cursor.position() + len as u64);
        }
    }
}

fn handle_packet(bytes: &[u8]) {
    let data = match Packets::decode(bytes) {
        Some(p) => p,
        None => return,
    };

    match data {
        Packets::Hello(p) => {
            println!("Connected to minecraft {}", p.address);
        }
        Packets::Log(log) => {
            println!("{}", log.msg);
        }
        Packets::Skin(e) => {
            gen_skin(e);
        }
    }
}

fn gen_skin(skin: SkinPacket) {
    let buff = base64::decode(skin.data).unwrap();

    let mut decoder = Decoder::new(&buff[..]);
    let mut decoded_data = Vec::new();
    decoder.read_to_end(&mut decoded_data).unwrap();

    image::save_buffer(
        &std::path::Path::new(&format!("./skins/{}.png", skin.runtime_id)),
        &decoded_data[..],
        skin.width,
        skin.height,
        image::ColorType::Rgba8,
    )
    .unwrap();

    println!("Stole {}'s skin", skin.name);
}
