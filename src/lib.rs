extern crate winapi;
use std::thread;

use nex::Nex;
use winapi::shared::minwindef::*;
use winapi::ctypes::*;
use winapi::um::consoleapi;

mod utils;
mod data;
mod types;
mod nex;

use data::signatures;
use types::controller::Controller;

const NEX: Nex = Nex::new();

pub fn keyboard_listener() -> u32 {
    println!("Keyboard listener started");
    let s_offset = utils::find_signature("minecraft.windows.exe", signatures::KEYBOARD_LISTENER_SIG);
    if s_offset != 0x0 {
        let offset = s_offset+3;
        let keymap = s_offset + offset + 7;
    } else {
        println!("Failed to locate keymap.");
        panic!("Keymap not found");
    }
    let mouse_controller: Controller = g_data.getController();


    0
}

pub fn start() {
    unsafe { consoleapi::AllocConsole(); }
    println!("Dll injected");
    println!("Starting rime...");

    thread::spawn(keyboard_listener);

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
