use std::{ffi::CString, mem::size_of, ptr::null_mut};

use winapi::{
    ctypes::c_void,
    shared::minwindef::{HMODULE, LPVOID},
    um::{
        errhandlingapi,
        libloaderapi::*,
        memoryapi,
        processthreadsapi::GetCurrentProcess,
        psapi::{GetModuleInformation, MODULEINFO},
        winnt::{LPCSTR, PAGE_EXECUTE_READWRITE},
    },
};

use crate::types::actor::Actor;

use super::super::super::GAME_DATA;
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
        let offset: u64 = 0x04237058 + 0x70 + 0x0 + 0x18 + 0x88 + 0xAD8 + 0x0 + 0xF40 + 0x1D8;

        unsafe {
            let base: u64 =
                GetModuleHandleA(CString::new("Minecraft.Windows.exe").unwrap().as_ptr() as LPCSTR)
                    as u64;
            println!("Found base: {:X}", &base);
            self.jump_addr = base + offset; // TODO: Doesnt give me the correct address... unless its the ptr addr but idk how to use that yet
                                            // self.jump_addr = 0x228A3B9AA28;
            println!("Found jump address: {:X}", self.jump_addr);
            if memoryapi::VirtualProtect(
                self.jump_addr as LPVOID,
                size_of::<i32>(),
                PAGE_EXECUTE_READWRITE,
                &mut self.old_protect,
            ) == 0
            {
                println!(
                    "Failed to unprotect memory: {}",
                    errhandlingapi::GetLastError()
                )
            }
        }
    }
    fn on_tick(&mut self) {
        if self.active {
            let x: *const *mut i32;
            unsafe {
                x = self.jump_addr as *const *mut i32;
                // if *GAME_DATA.get_client_instance().unwrap().on_ground() {

                // }
                **x = 167772473;
            }
        }
    }
}
