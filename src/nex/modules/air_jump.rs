use std::{ffi::CString, mem::size_of, ptr::null_mut};

use winapi::{
    shared::minwindef::{HMODULE, LPVOID},
    um::{
        libloaderapi::*,
        memoryapi,
        processthreadsapi::GetCurrentProcess,
        psapi::{GetModuleInformation, MODULEINFO},
        winnt::{LPCSTR, PAGE_EXECUTE_READWRITE},
        errhandlingapi,
    }, ctypes::c_void,
};

use super::super::module_handler::Module;
pub struct AirJump {
    active: bool,
    jump_addr: u64,
    old_protect: u32,
}

impl AirJump {
    pub fn new() -> Self {
        Self {
            active: false,
            jump_addr: 0,
            old_protect: 0,
        }
    }
}

impl Module for AirJump {
    fn on_enable(&mut self) {
        self.active = true;
        let offset: u64 = 0x04237058;

        unsafe {
            let base: u64 = GetModuleHandleA(CString::new("Minecraft.Windows.exe").unwrap().as_ptr() as LPCSTR) as u64;
            println!("Found base: {:X}", &base);
            // self.jump_addr = base + offset; // TODO: Doesnt give me the correct address... unless its the ptr addr but idk how to use that yet
            self.jump_addr = 0x2450D1ADA28;
            println!("Found jump address: {:X}", self.jump_addr);
            if memoryapi::VirtualProtect(
                self.jump_addr as LPVOID,
                size_of::<i32>(),
                PAGE_EXECUTE_READWRITE,
                &mut self.old_protect,
            ) == 0
            {
                println!("Failed to unprotect memory: {}", errhandlingapi::GetLastError())
            }
        }
    }
    fn on_tick(&mut self) {
        // self.jump_addr = 16777237;
        let buf: *const c_void = 167772473 as *const c_void;
        let bytes_written: *mut usize = null_mut();
        // let tmp_addr: *mut c_void = self.jump_addr as *mut c_void;
        unsafe {
            if memoryapi::WriteProcessMemory(GetCurrentProcess(), self.jump_addr as LPVOID, buf, size_of::<i32>(), bytes_written) == 0 {
                println!("Failed to write memory: {}", errhandlingapi::GetLastError()) // TODO: This throws error code 998 | 0x3E6
            }
        }
    }
}
