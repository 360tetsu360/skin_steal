use crate::{memory, network::skin, sdk::TextHolder};
use lazy_static::lazy_static;
use libflate::deflate::Encoder;
use minhook::MhError;
use std::{io::Write, sync::Mutex};

type PlayerTick = unsafe extern "fastcall" fn(*const u64, u64, u64);
unsafe extern "fastcall" fn _callback(_: *const u64, _: u64, _: u64) {}
static mut BACK_TO_FUNCTION: PlayerTick = _callback;

unsafe extern "fastcall" fn entt_callback(player: *const u64, _a1: u64, _a2: u64) {
    let runtime = player as u64;
    let name =
        match (*std::mem::transmute::<u64, *const TextHolder>(player as u64 + 0x8c0)).get_text() {
            Ok(p) => p,
            Err(_) => "anonymous player".to_string(),
        };
    let width = *std::mem::transmute::<u64, *const u32>(player as u64 + 0xc1c);
    let height = *std::mem::transmute::<u64, *const u32>(player as u64 + 0xc20);
    skin_manage(
        runtime,
        name,
        width,
        height,
        *((player as u64 + 0xc30) as *const *const u8),
    );
    BACK_TO_FUNCTION(player, _a1, _a2);
}

lazy_static! {
    static ref STOLEN: Mutex<Vec<u64>> = Mutex::new(vec![]);
}
unsafe fn skin_manage(runtime: u64, name: String, width: u32, height: u32, data: *const u8) {
    if !STOLEN.lock().unwrap().contains(&runtime) && width == 64 && height == 64 {
        let skin_data = std::slice::from_raw_parts(data, 0x4000); //64px * 64px * 4(rgba)
        let mut encoder = Encoder::new(Vec::new());
        encoder.write_all(skin_data).unwrap();
        let encoded_data = encoder.finish().into_result().unwrap();
        let base64ed = base64::encode(encoded_data);

        skin(runtime, name, width, height, &base64ed);

        STOLEN.lock().unwrap().push(runtime);
    }
}

pub unsafe fn install_hooks() -> Result<(), MhError> {
    minhook::initialize()?;

    let fnpointer =
        memory::find_signature("F3 0F ?? ?? ?? ?? 00 00 ?? 0F ?? 00 F3 0F ?? ?? F3 0F ?? ?? 04"); //not working function
    let cc = std::mem::transmute::<*mut PlayerTick, *mut *const ()>(&mut BACK_TO_FUNCTION);
    minhook::create_hook(fnpointer as *const (), entt_callback as *const (), cc)?;
    minhook::enable_hook(fnpointer as *const ())?;
    Ok(())
}
