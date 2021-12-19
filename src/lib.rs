extern crate winapi;
use std::thread;

use winapi::shared::minwindef::*;
use winapi::ctypes::*;
use winapi::um::consoleapi;

mod utils;

pub fn keyboard_listener(lpParam: LPVOID) -> u32 {
    println!("Keyboard listener created");
    if let s_offset: i32 = utils::find_signature("") != 0x0 {
        let offset = s_offset+3;
        let keymap = s_offset + offset + 7;

    }
}

pub fn start() {
    unsafe { consoleapi::AllocConsole(); }
    println!("Dll injected")
}

#[no_mangle]
pub extern "stdcall" fn DllMain(
    h_inst: HINSTANCE,
    reason: u32,
    _: LPVOID
) {
    match reason {
        1 => {
            start();
        }
        0 => {

        }
        _ => {

        }
    }
}
