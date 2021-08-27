use std::{
    error::Error,
    ffi::{CStr, CString},
    ops::Add,
};

use windows_bindings::Windows::Win32::{
    Foundation::{BOOL, HINSTANCE, PSTR},
    System::{
        Console::{AllocConsole, FreeConsole},
        Diagnostics::Debug::GetLastError,
        LibraryLoader::{FreeLibraryAndExitThread, GetModuleHandleA},
        ProcessStatus::{K32GetModuleInformation, MODULEINFO},
        Threading::GetCurrentProcess,
    },
};

pub fn start(module: HINSTANCE) {
    // Debug console
    open_debug_console().unwrap();

    println!("Hello from DLL!");

    // Example of getting base address
    /*let process_base_address = unsafe { GetModuleHandleA(PSTR(std::ptr::null_mut())) };
    println!("Test: {:#01x}", process_base_address.0);
    let p = process_base_address.0 as *const u32;
    let n = unsafe { *p };
    println!("{}", n);*/

    // Example of pattern scanning
    /*let my_cool_pattern = pattern_scan("notepad.exe", "? 15 7B 34 02 ? 48").unwrap();
    println!("{:#01x}", my_cool_pattern);*/

    // TODO: Detouring
}

// Sample pattern scanning reference:
// https://www.unknowncheats.me/forum/league-of-legends/424623-internal-pattern-scanning.html

fn pattern_scan(str: &str, pattern_and_mask: &str) -> Result<usize, Box<dyn Error>> {
    let module = get_module_info(str).expect("failed getting module");

    let base = module.lpBaseOfDll as *mut u8;
    let size = module.SizeOfImage as usize;

    let mut mask = Vec::new();
    let mut pattern = Vec::new();

    let pattern_and_mask_string = String::from(pattern_and_mask);

    for str in pattern_and_mask_string.split(" ") {
        if str.chars().count() > 2 {
            Err(format!("Element {}'s length is greater than 2", str))?
        }

        if str == "??" {
            Err("Only a single question mark should be used")?
        }

        mask.push(str);

        if str == "?" {
            pattern.push(0);
        } else {
            pattern.push(u8::from_str_radix(str, 16).unwrap());
        }
    }

    for i in 0..size - pattern.len() {
        let mut found = true;

        for j in 0..pattern.len() {
            let mask_char = *mask.get(j).unwrap();
            let pattern_byte = *pattern.get(j).unwrap();

            if mask_char != "?" && pattern_byte != unsafe { *base.add(i + j) } {
                found = false;
                break;
            };
        }

        if found {
            return Ok(unsafe { base.add(i) } as usize);
        }
    }

    Err("failed finding signature")?
}

fn get_module_info(module: &str) -> Option<MODULEINFO> {
    let mut awer: MODULEINFO = MODULEINFO {
        lpBaseOfDll: std::ptr::null_mut(),
        SizeOfImage: 0,
        EntryPoint: std::ptr::null_mut(),
    };

    let test = CString::new(module).unwrap();

    let raw = test.into_raw() as *mut u8;

    let h_module = unsafe { GetModuleHandleA(PSTR(raw)) };

    if (h_module.0) == 0 {
        None
    } else {
        unsafe {
            K32GetModuleInformation(
                GetCurrentProcess(),
                h_module,
                &mut awer as *mut MODULEINFO,
                std::mem::size_of::<MODULEINFO>() as u32,
            );
        }
        Some(awer)
    }
}

fn pattern_scan_multithread() {
    //pattern_scan_actual(num_cpus::get())
}

fn pattern_scan_singlethread() {
    //pattern_scan_actual(1)
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
        Err(format!(
            "failed closing console, GetLastError: {}",
            unsafe { GetLastError() }.0
        ))?
    } else {
        Ok(())
    }
}

fn close_cheat(module: HINSTANCE) {
    unsafe {
        FreeLibraryAndExitThread(module, 0);
    }
}
