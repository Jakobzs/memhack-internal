use std::error::Error;

use windows_bindings::Windows::Win32::{
    Foundation::{BOOL, HINSTANCE, PSTR},
    System::{
        Console::{AllocConsole, FreeConsole},
        Diagnostics::Debug::GetLastError,
        LibraryLoader::{FreeLibraryAndExitThread, GetModuleHandleA},
    },
};

pub fn start(module: HINSTANCE) {
    // Debug console
    open_debug_console().unwrap();

    println!("Hello from DLL!");

    let process_base_address = unsafe { GetModuleHandleA(PSTR(std::ptr::null_mut())) };

    println!("Test: {:#01x}", process_base_address.0);

    let p = process_base_address.0 as *const u32;
    let n = unsafe { *p };

    println!("{}", n);

    close_cheat(module);
}

fn open_debug_console() -> Result<(), Box<dyn Error>> {
    if unsafe { AllocConsole() }.0 == 0 {
        Err(format!(
            "failed opening console, GetLastError: {}",
            unsafe { GetLastError() }.0
        ))?
    } else {
        Ok(())
    }
}

fn close_debug_console() -> Result<(), Box<dyn Error>> {
    if unsafe { FreeConsole() }.0 == 0 {
        Err("failed closing console")?
    } else {
        Ok(())
    }
}

fn close_cheat(module: HINSTANCE) {
    unsafe {
        FreeLibraryAndExitThread(module, 0);
    }
}
