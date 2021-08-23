use std::os::windows::process;

use windows_bindings::Windows::Win32::{
    Foundation::PSTR,
    System::{Console::AllocConsole, LibraryLoader::GetModuleHandleA},
};

pub fn start() {
    // Debug console
    unsafe { AllocConsole() };

    println!("Hello from DLL!");

    let process_base_address = unsafe { GetModuleHandleA(PSTR(std::ptr::null_mut())) };

    println!("Test: {:#01x}", process_base_address.0);

    let p = process_base_address.0 as *const u32;
    let n = unsafe { std::ptr::read(p) };

    println!("{}", n);
}
