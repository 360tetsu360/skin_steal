#[repr(C)]
pub struct EntityList {
    pub _pad: [u8; 0x70],
    pub first_entity: u64, //0x0050
    pub last_entity: u64,  //0x0058
    pub end_addr: u64,     //0x0060
}

/*
pub union Text {
    pub inline_text : [u8;16],
    pub text_addr : *const u8
}

#[repr(C)]
pub struct TextHolder {
    pub data : Text,
    pub text_length : usize,
    pub aligned_text_length : usize,
}

impl TextHolder {
    pub unsafe fn get_text(&self) -> Result<String, FromUtf8Error> {
        if self.aligned_text_length < 0x10 {
            return String::from_utf8((&self.data.inline_text[..self.text_length]).to_vec())
        }else {
            let text = std::slice::from_raw_parts(self.data.text_addr, self.aligned_text_length);
            return String::from_utf8(text.to_vec())
        }
    }
}
*/
