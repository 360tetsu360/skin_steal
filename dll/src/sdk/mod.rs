use std::string::FromUtf8Error;
pub union Text {
    pub inline_text: [u8; 16],
    pub text_addr: *const u8,
}

#[repr(C)]
pub struct TextHolder {
    pub data: Text,
    pub text_length: usize,
    pub aligned_text_length: usize,
}

impl TextHolder {
    pub unsafe fn get_text(&self) -> Result<String, FromUtf8Error> {
        if self.aligned_text_length < 0x10 {
            String::from_utf8((&self.data.inline_text[..self.text_length]).to_vec())
        } else {
            let text = std::slice::from_raw_parts(self.data.text_addr, self.aligned_text_length);
            String::from_utf8(text.to_vec())
        }
    }
}
