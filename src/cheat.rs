use std::ptr;

use windows_bindings::Windows::Win32::System::Console::AllocConsole;

pub fn start() {
    // Debug console
    unsafe { AllocConsole() };

    println!("Hello from DLL!");

    let p = 0x282A6325270 as *const u32;
    let n = unsafe { ptr::read(p) };

    println!("{}", n);
}
