use serde::{Deserialize, Serialize};

pub enum Packets {
    Hello(HelloPacket),
    Log(LogPacket),
    Skin(SkinPacket),
}

impl Packets {
    pub fn decode(payload: &[u8]) -> Option<Self> {
        if payload.len() < 2 {
            return None;
        }

        let id = payload[0];

        let data_str = match String::from_utf8(payload[1..].to_vec()) {
            Ok(p) => p,
            Err(_) => return None,
        };

        match id {
            0 => match serde_json::from_str::<HelloPacket>(&data_str) {
                Ok(p) => Some(Self::Hello(p)),
                Err(_) => None,
            },
            1 => match serde_json::from_str::<LogPacket>(&data_str) {
                Ok(p) => Some(Self::Log(p)),
                Err(_) => None,
            },
            2 => match serde_json::from_str::<SkinPacket>(&data_str) {
                Ok(p) => Some(Self::Skin(p)),
                Err(_) => None,
            },
            _ => None,
        }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut ret = vec![];
        match self {
            Packets::Hello(p) => {
                ret.push(0);
                let mut json = serde_json::to_string(p).unwrap().as_bytes().to_vec();
                ret.append(&mut json)
            }
            Packets::Log(p) => {
                ret.push(1);
                let mut json = serde_json::to_string(p).unwrap().as_bytes().to_vec();
                ret.append(&mut json)
            }
            Packets::Skin(p) => {
                ret.push(2);
                let mut json = serde_json::to_string(p).unwrap().as_bytes().to_vec();
                ret.append(&mut json)
            }
        }
        ret
    }
}

#[derive(Serialize, Deserialize)]
pub struct HelloPacket {
    pub address: String,
}

#[derive(Serialize, Deserialize)]
pub struct LogPacket {
    pub msg: String,
}

#[derive(Serialize, Deserialize)]
pub struct SkinPacket {
    pub runtime_id: u64,
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub data: String,
}
