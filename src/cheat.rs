use std::error::Error;

use windows_bindings::Windows::Win32::{
    Foundation::{BOOL, PSTR},
    System::{
        Console::{AllocConsole, FreeConsole},
        Diagnostics::Debug::GetLastError,
        LibraryLoader::GetModuleHandleA,
    },
};

pub fn start() {
    // Debug console
    open_debug_console().unwrap();

    println!("Hello from DLL!");

    let process_base_address = unsafe { GetModuleHandleA(PSTR(std::ptr::null_mut())) };

    println!("Test: {:#01x}", process_base_address.0);

    let p = process_base_address.0 as *const u32;
    let n = unsafe { *p };

    println!("{}", n);
}

fn open_debug_console() -> Result<(), Box<dyn Error>> {
    if unsafe { AllocConsole() }.0 == 0 {
        let testy = unsafe { GetLastError() };

        let test = testy.0;

        Err("failed opening console, GetLastError")?
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
