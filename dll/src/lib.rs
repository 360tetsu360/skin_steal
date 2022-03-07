use hook::install_hooks;
use network::{init, send};
use winapi::{
    ctypes::c_void,
    shared::minwindef::{BOOL, DWORD, HINSTANCE, HINSTANCE__, LPVOID, TRUE},
    um::processthreadsapi::CreateThread,
};
mod hook;
mod memory;
mod network;
mod sdk;

const DLL_PROCESS_DETACH: DWORD = 0;
const DLL_PROCESS_ATTACH: DWORD = 1;
const DLL_THREAD_ATTACH: DWORD = 2;
const DLL_THREAD_DETACH: DWORD = 3;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn DllMain(hinst: HINSTANCE, reason: DWORD, _reserved: LPVOID) -> BOOL {
    match reason {
        DLL_PROCESS_DETACH => {}
        DLL_PROCESS_ATTACH => unsafe {
            CreateThread(
                std::ptr::null_mut(),
                0,
                Some(start),
                std::mem::transmute::<*mut HINSTANCE__, *mut c_void>(hinst),
                0,
                std::ptr::null_mut(),
            );
        },
        DLL_THREAD_ATTACH => {}
        DLL_THREAD_DETACH => {}
        _ => {}
    };

    TRUE
}

unsafe extern "system" fn start(_dll: *mut c_void) -> DWORD {
    install_hooks().unwrap();
    init();
    send("Hello Logger!");
    0
}
