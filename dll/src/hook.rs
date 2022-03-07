use minhook::MhError;

use crate::{memory, sdk::EntityList};

type EnttListTick = unsafe extern "fastcall" fn(*mut EntityList);
unsafe extern "fastcall" fn _callback(_: *mut EntityList) {}
static mut BACK_TO_FUNCTION: EnttListTick = _callback;

unsafe extern "fastcall" fn entt_callback(enttlist: *mut EntityList) {
    BACK_TO_FUNCTION(enttlist);
}

pub unsafe fn install_hooks() -> Result<(), MhError> {
    minhook::initialize()?;

    let fnpointer = memory::find_signature("48 89 ?? ?? ?? 57 48 83 EC ?? 48 8B ?? E8 ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? 48 8B D8 ?? ?? ?? ?? ?? ?? 48 99"); //not working function
    let cc = std::mem::transmute::<*mut EnttListTick, *mut *const ()>(&mut BACK_TO_FUNCTION);
    minhook::create_hook(fnpointer as *const (), entt_callback as *const (), cc)?;
    minhook::enable_hook(fnpointer as *const ())?;
    Ok(())
}
