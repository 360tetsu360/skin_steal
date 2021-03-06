pub const MH_STATUS_MH_UNKNOWN: MH_STATUS = -1;
pub const MH_STATUS_MH_OK: MH_STATUS = 0;
pub const MH_STATUS_MH_ERROR_ALREADY_INITIALIZED: MH_STATUS = 1;
pub const MH_STATUS_MH_ERROR_NOT_INITIALIZED: MH_STATUS = 2;
pub const MH_STATUS_MH_ERROR_ALREADY_CREATED: MH_STATUS = 3;
pub const MH_STATUS_MH_ERROR_NOT_CREATED: MH_STATUS = 4;
pub const MH_STATUS_MH_ERROR_ENABLED: MH_STATUS = 5;
pub const MH_STATUS_MH_ERROR_DISABLED: MH_STATUS = 6;
pub const MH_STATUS_MH_ERROR_NOT_EXECUTABLE: MH_STATUS = 7;
pub const MH_STATUS_MH_ERROR_UNSUPPORTED_FUNCTION: MH_STATUS = 8;
pub const MH_STATUS_MH_ERROR_MEMORY_ALLOC: MH_STATUS = 9;
pub const MH_STATUS_MH_ERROR_MEMORY_PROTECT: MH_STATUS = 10;
pub const MH_STATUS_MH_ERROR_MODULE_NOT_FOUND: MH_STATUS = 11;
pub const MH_STATUS_MH_ERROR_FUNCTION_NOT_FOUND: MH_STATUS = 12;

#[allow(non_camel_case_types)]
pub type MH_STATUS = ::std::os::raw::c_int;

#[link(name = "libMinHook", kind = "static")]
extern "C" {
    pub fn MH_Initialize() -> MH_STATUS;

    pub fn MH_Uninitialize() -> MH_STATUS;

    pub fn MH_CreateHook(
        pTarget: *const (),
        pDetour: *const (),
        ppOriginal: *mut *const (),
    ) -> MH_STATUS;

    pub fn MH_CreateHookApi(
        pszModule: *const u16, //LPWSTR
        pszProcName: *const u8,
        pDetour: *const (),
        ppOriginal: *mut *const (),
    ) -> MH_STATUS;

    pub fn MH_CreateHookApiEx(
        pszModule: *const u16,
        pszProcName: *const u8,
        pDetour: *const (),
        ppOriginal: *mut *const (),
        ppTarget: *mut *const (),
    ) -> MH_STATUS;

    pub fn MH_RemoveHook(pTarget: *const ()) -> MH_STATUS;

    pub fn MH_EnableHook(pTarget: *const ()) -> MH_STATUS;

    pub fn MH_DisableHook(pTarget: *const ()) -> MH_STATUS;

    pub fn MH_QueueEnableHook(pTarget: *const ()) -> MH_STATUS;

    pub fn MH_QueueDisableHook(pTarget: *const ()) -> MH_STATUS;

    pub fn MH_ApplyQueued() -> MH_STATUS;

    #[allow(dead_code)]
    pub fn MH_StatusToString(status: MH_STATUS) -> *const ::std::os::raw::c_char;
}
