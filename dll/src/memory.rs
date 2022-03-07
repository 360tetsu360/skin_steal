use std::mem;
use std::ptr::null_mut;
use winapi::um::{
    libloaderapi::GetModuleHandleA,
    processthreadsapi::GetCurrentProcess,
    psapi::{GetModuleInformation, MODULEINFO},
};

unsafe fn get_mod_base() -> u64 {
    GetModuleHandleA(std::ptr::null_mut()) as u64
}

unsafe fn get_mod_size() -> u64 {
    let mut info = MODULEINFO {
        lpBaseOfDll: null_mut(),
        SizeOfImage: 0,
        EntryPoint: null_mut(),
    };
    GetModuleInformation(
        GetCurrentProcess(),
        GetModuleHandleA(std::ptr::null_mut()),
        &mut info,
        mem::size_of::<MODULEINFO>() as u32,
    );
    info.SizeOfImage.into()
}

unsafe fn get_mod_end() -> u64 {
    get_mod_base() + get_mod_size()
}

pub unsafe fn find_signature(pattern: &str) -> u64 {
    let mut pos: usize = 0;
    let mut first_match: u64 = 0;
    for p_cur in get_mod_base()..get_mod_end() {
        let _ = *mem::transmute::<u64, *const u8>(p_cur);
        if pos > pattern.len() {
            return first_match;
        }
        if pattern.as_bytes()[pos] == b'?'
            || *mem::transmute::<u64, *const u8>(p_cur) == get_byte(pattern, pos)
        {
            if first_match == 0 {
                first_match = p_cur;
            }
            if pos + 1 == pattern.len() {
                return first_match;
            }
            pos += 3;
        } else {
            pos = 0;
            first_match = 0;
        }
    }
    0
}

fn get_byte(stri: &str, pos: usize) -> u8 {
    u8::from_str_radix(&stri[pos..pos + 2], 16).unwrap()
}
