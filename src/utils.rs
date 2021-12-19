use std::mem::size_of;

use winapi::um::psapi::{GetModuleInformation, MODULEINFO};
use winapi::um::libloaderapi::GetModuleHandleA;
use winapi::{um::processthreadsapi::GetCurrentProcess, shared::minwindef::HMODULE};

pub fn find_signature(szModule: *const i8, szSignature: *const i8) -> u32 {
    let pattern = szSignature;
    let firstMatch = 0;
    let rangeStart = unsafe { GetModuleHandleA(szModule) };
    let miModInfo: *mut MODULEINFO;
    let mut init: bool = false;

    if !init {
        init = true;
        unsafe { 
            GetModuleInformation(GetCurrentProcess(), rangeStart as HMODULE, miModInfo, size_of::<HMODULE>() as u32)
        };
        ()
    }
    0
}