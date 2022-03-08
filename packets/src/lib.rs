use serde::{Deserialize, Serialize};

pub enum Packets {
    Hello(HelloPacket),
    Log(LogPacket),
    SkinHeader(SkinHeaderPacket),
    SkinPayload(SkinPayloadPacket),
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
            2 => match serde_json::from_str::<SkinHeaderPacket>(&data_str) {
                Ok(p) => Some(Self::SkinHeader(p)),
                Err(_) => None,
            },
            3 => match serde_json::from_str::<SkinPayloadPacket>(&data_str) {
                Ok(p) => Some(Self::SkinPayload(p)),
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
            Packets::SkinHeader(p) => {
                ret.push(2);
                let mut json = serde_json::to_string(p).unwrap().as_bytes().to_vec();
                ret.append(&mut json)
            }
            Packets::SkinPayload(p) => {
                ret.push(3);
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
pub struct SkinHeaderPacket {
    pub runtime_id: u64,
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub packet_id: u32,
    pub len: u32,
}

#[derive(Serialize, Deserialize)]
pub struct SkinPayloadPacket {
    pub packet_id: u32,
    pub index: u32,
    pub data: String,
}
