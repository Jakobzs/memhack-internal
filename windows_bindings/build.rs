fn main() {
    windows::build!(
        Windows::Win32::System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH, DLL_THREAD_ATTACH, DLL_THREAD_DETACH},
        Windows::Win32::Foundation::{BOOL, HINSTANCE, PSTR},
        Windows::Win32::System::Console::{AllocConsole, FreeConsole},
        Windows::Win32::System::LibraryLoader::GetModuleHandleA,
        Windows::Win32::System::Diagnostics::Debug::GetLastError,
        Windows::Win32::System::LibraryLoader::FreeLibraryAndExitThread,
        Windows::Win32::System::ProcessStatus::{MODULEINFO, K32GetModuleInformation},
        Windows::Win32::System::Threading::GetCurrentProcess,
    );
}
