use std::ffi::c_void;

use windows::Win32::Foundation::{BOOL, HINSTANCE};

mod cheat;

#[no_mangle]
#[allow(unused_variables)]
pub extern "stdcall" fn DllMain(module: HINSTANCE, reason: u32, reserved: *mut c_void) -> BOOL {
    match reason {
        DLL_PROCESS_ATTACH => cheat::start(module),
        DLL_THREAD_ATTACH => (),
        DLL_THREAD_DETACH => (),
        DLL_PROCESS_DETACH => (),
        _ => (),
    }

    BOOL::from(true)
}
