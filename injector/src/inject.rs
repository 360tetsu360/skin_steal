use std::cmp::Ordering;
use std::ffi::OsStr;
use std::os::windows::prelude::OsStrExt;

use std::str;
use winapi::ctypes::c_void;
use winapi::shared::minwindef::{DWORD, HLOCAL, MAX_PATH};
use winapi::shared::sddl::ConvertStringSidToSidW;
use winapi::shared::winerror::ERROR_SUCCESS;
use winapi::um::accctrl::SET_ACCESS;
use winapi::um::accctrl::{
    EXPLICIT_ACCESSW, SE_FILE_OBJECT, SUB_CONTAINERS_AND_OBJECTS_INHERIT, TRUSTEE_IS_SID,
    TRUSTEE_IS_WELL_KNOWN_GROUP, TRUSTEE_W,
};
use winapi::um::aclapi::{GetNamedSecurityInfoW, SetEntriesInAclW, SetNamedSecurityInfoW};
use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
use winapi::um::libloaderapi::{GetModuleHandleW, GetProcAddress};
use winapi::um::memoryapi::{VirtualAllocEx, WriteProcessMemory};
use winapi::um::processthreadsapi::{CreateRemoteThread, OpenProcess};
use winapi::um::tlhelp32::{
    CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32, TH32CS_SNAPPROCESS,
};
use winapi::um::winbase::LocalFree;
use winapi::um::winnt::{
    ACL, DACL_SECURITY_INFORMATION, GENERIC_EXECUTE, GENERIC_READ, HANDLE, MEM_COMMIT, MEM_RESERVE,
    PAGE_READWRITE, PROCESS_ALL_ACCESS, PSECURITY_DESCRIPTOR, PSID, SECURITY_INFORMATION,
};

fn str_wide_vec(str: &str) -> Vec<u16> {
    return OsStr::new(str)
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect();
}

unsafe fn to_string(raw: &[i8]) -> Vec<u8> {
    let i = &*(raw as *const [i8] as *const [u8]);
    let mut ret: Vec<u8> = vec![];
    for char in i {
        if *char == 0 {
            break;
        }
        ret.push(*char);
    }
    ret
}

pub unsafe fn get_proc_id(proc_name: &str) -> u32 {
    let mut proc_id = 0;
    let h_snap = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
    if h_snap != INVALID_HANDLE_VALUE {
        let mut procentry = PROCESSENTRY32 {
            dwSize: 0,
            cntUsage: 0,
            th32ProcessID: 0,
            th32DefaultHeapID: 0,
            th32ModuleID: 0,
            cntThreads: 0,
            th32ParentProcessID: 0,
            pcPriClassBase: 0,
            dwFlags: 0,
            szExeFile: [0i8; MAX_PATH],
        };
        procentry.dwSize = std::mem::size_of::<PROCESSENTRY32>() as u32;

        if Process32First(h_snap, &mut procentry) != 0 {
            let vec: &[u8] = &to_string(&procentry.szExeFile);
            if vec.cmp(proc_name.as_bytes()) == Ordering::Equal {
                return procentry.th32ProcessID;
            }
            while Process32Next(h_snap, &mut procentry) != 0 {
                let vec: &[u8] = &to_string(&procentry.szExeFile);
                if vec.cmp(proc_name.as_bytes()) == Ordering::Equal {
                    proc_id = procentry.th32ProcessID;
                    break;
                }
            }
        }
    }
    CloseHandle(h_snap);
    proc_id
}

pub unsafe fn set_access_control(executable_name: &str, access_string: &str) -> bool {
    let mut security_descriptor: PSECURITY_DESCRIPTOR = std::ptr::null_mut();
    let mut explicit_access: EXPLICIT_ACCESSW = winapi::um::accctrl::EXPLICIT_ACCESS_W {
        grfAccessPermissions: GENERIC_READ | GENERIC_EXECUTE,
        grfAccessMode: SET_ACCESS,
        grfInheritance: SUB_CONTAINERS_AND_OBJECTS_INHERIT,
        Trustee: TRUSTEE_W {
            pMultipleTrustee: std::ptr::null_mut(),
            MultipleTrusteeOperation: 0,
            TrusteeForm: TRUSTEE_IS_SID,
            TrusteeType: TRUSTEE_IS_WELL_KNOWN_GROUP,
            ptstrName: std::ptr::null_mut(),
        },
    };

    let mut access_control_current: *mut ACL = std::ptr::null_mut();
    let mut access_control_new: *mut ACL = std::ptr::null_mut();

    let security_info: SECURITY_INFORMATION = DACL_SECURITY_INFORMATION;
    let mut security_identifier: PSID = std::ptr::null_mut();

    if GetNamedSecurityInfoW(
        str_wide_vec(executable_name).as_ptr(),
        SE_FILE_OBJECT,
        DACL_SECURITY_INFORMATION,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        &mut access_control_current,
        std::ptr::null_mut(),
        &mut security_descriptor,
    ) == ERROR_SUCCESS
    {
        ConvertStringSidToSidW(
            str_wide_vec(access_string).as_ptr(),
            &mut security_identifier,
        );
        if !security_identifier.is_null() {
            explicit_access.Trustee.ptstrName =
                std::mem::transmute::<PSID, *mut u16>(security_identifier);

            if SetEntriesInAclW(
                1,
                &mut explicit_access,
                access_control_current,
                &mut access_control_new,
            ) == ERROR_SUCCESS
            {
                SetNamedSecurityInfoW(
                    str_wide_vec(executable_name).as_mut_ptr(),
                    SE_FILE_OBJECT,
                    security_info,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    access_control_new,
                    std::ptr::null_mut(),
                );
            } else {
                return false;
            }
        }
    } else {
        return false;
    }

    if !security_descriptor.is_null() {
        LocalFree(std::mem::transmute::<PSECURITY_DESCRIPTOR, HLOCAL>(
            security_descriptor,
        ));
    }
    if !access_control_new.is_null() {
        LocalFree(std::mem::transmute::<*mut ACL, HLOCAL>(access_control_new));
    }

    true
}

pub unsafe fn inject_dll(proc_id: DWORD, dll_path: &str) -> bool {
    let h_proc: HANDLE = OpenProcess(PROCESS_ALL_ACCESS, 0, proc_id);

    if !h_proc.is_null() && h_proc != INVALID_HANDLE_VALUE {
        let loc = VirtualAllocEx(
            h_proc,
            std::ptr::null_mut(),
            MAX_PATH,
            MEM_COMMIT | MEM_RESERVE,
            PAGE_READWRITE,
        );

        WriteProcessMemory(
            h_proc,
            loc,
            str_wide_vec(dll_path).as_ptr() as *const winapi::ctypes::c_void,
            str_wide_vec(dll_path).len() * 2 + 2,
            std::ptr::null_mut(),
        ); //length * 2 for bytes + 2 for end string

        let kernel32 = GetModuleHandleW(str_wide_vec("Kernel32.dll").as_ptr());
        let load_library_w = GetProcAddress(kernel32, b"LoadLibraryW\0".as_ptr() as *const i8);

        if load_library_w.is_null() {
            return false;
        }

        type ThreadStartRoutine = unsafe extern "system" fn(*mut c_void) -> DWORD;
        let start_routine: ThreadStartRoutine = std::mem::transmute(load_library_w);

        let h_thread: HANDLE = CreateRemoteThread(
            h_proc,
            std::ptr::null_mut(),
            0,
            Some(start_routine),
            loc,
            0,
            std::ptr::null_mut(),
        );

        if !h_thread.is_null() {
            CloseHandle(h_thread);
        } else {
            return false;
        }
    } else {
        return false;
    }
    if !h_proc.is_null() {
        CloseHandle(h_proc);
    }
    true
}

pub unsafe fn inject(target: &str, dll: &str) -> bool {
    let proc_id = get_proc_id(target);
    if proc_id == 0 {
        return false;
    }
    println!("Minecraft PID : {}", proc_id);

    if !set_access_control(dll, "S-1-15-2-1") {
        return false;
    }
    inject_dll(proc_id, dll)
}
