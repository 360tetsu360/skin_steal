use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

use minhook::*;

pub mod error;
mod minhook;
pub use error::*;

#[allow(clippy::missing_safety_doc)]
fn parse_status(status: MH_STATUS) -> Result<(), MhError> {
    if status == MH_STATUS_MH_OK {
        Ok(())
    } else {
        Err(MhError::from(status))
    }
}

#[allow(clippy::missing_safety_doc)]
fn str_wide_vec(str: &str) -> Vec<u16> {
    return OsStr::new(str)
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect();
}

#[allow(clippy::missing_safety_doc)]
pub unsafe fn initialize() -> Result<(), MhError> {
    parse_status(MH_Initialize())
}

#[allow(clippy::missing_safety_doc)]
pub unsafe fn uninitialize() -> Result<(), MhError> {
    parse_status(MH_Uninitialize())
}

#[allow(clippy::missing_safety_doc)]
pub unsafe fn create_hook(
    p_target: *const (),
    p_detour: *const (),
    pp_original: *mut *const (),
) -> Result<(), MhError> {
    parse_status(MH_CreateHook(p_target, p_detour, pp_original))
}

#[allow(clippy::missing_safety_doc)]
pub unsafe fn create_hook_api(
    psz_module: &str,
    psz_proc_name: &str,
    p_detour: *const (),
    pp_original: *mut *const (),
) -> Result<(), MhError> {
    parse_status(MH_CreateHookApi(
        str_wide_vec(psz_module).as_ptr(),
        psz_proc_name.as_ptr(),
        p_detour,
        pp_original,
    ))
}

#[allow(clippy::missing_safety_doc)]
pub unsafe fn create_hook_api_ex(
    psz_module: &str,
    psz_proc_name: &str,
    p_detour: *const (),
    pp_original: *mut *const (),
    pp_target: *mut *const (),
) -> Result<(), MhError> {
    parse_status(MH_CreateHookApiEx(
        str_wide_vec(psz_module).as_ptr(),
        psz_proc_name.as_ptr(),
        p_detour,
        pp_original,
        pp_target,
    ))
}

#[allow(clippy::missing_safety_doc)]
pub unsafe fn remove_hook(p_target: *const ()) -> Result<(), MhError> {
    parse_status(MH_RemoveHook(p_target))
}

#[allow(clippy::missing_safety_doc)]
pub unsafe fn enable_hook(p_target: *const ()) -> Result<(), MhError> {
    parse_status(MH_EnableHook(p_target))
}

#[allow(clippy::missing_safety_doc)]
pub unsafe fn disable_hook(p_target: *const ()) -> Result<(), MhError> {
    parse_status(MH_DisableHook(p_target))
}

#[allow(clippy::missing_safety_doc)]
pub unsafe fn queue_enable_hook(p_target: *const ()) -> Result<(), MhError> {
    parse_status(MH_QueueEnableHook(p_target))
}

#[allow(clippy::missing_safety_doc)]
pub unsafe fn queue_disable_hook(p_target: *const ()) -> Result<(), MhError> {
    parse_status(MH_QueueDisableHook(p_target))
}

#[allow(clippy::missing_safety_doc)]
pub unsafe fn apply_queued() -> Result<(), MhError> {
    parse_status(MH_ApplyQueued())
}
