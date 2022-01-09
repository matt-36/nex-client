extern crate winapi;
use core::time;
use std::thread;

use nex::Nex;
use types::game_data;
use winapi::shared::minwindef::*;
use winapi::ctypes::*;
use winapi::um::consoleapi;
use winapi::um::memoryapi;

mod utils;
mod data;
mod types;
mod nex;




use data::offsets;
use types::controller;
use winapi::um::processthreadsapi::OpenProcess;

use crate::nex::module_handler::Module;
use crate::nex::module_handler::ModuleHandler;

const NEX: Nex = Nex::new(); // needs fixed
const GAME_DATA: game_data::GameData = game_data::GameData::new(); // needs fixed

pub fn keyboard_listener() -> u32 {
    println!("Keyboard listener started");
    let s_offset = 0xFF; //utils::find_signature("minecraft.windows.exe", signatures::KEYBOARD_LISTENER_SIG);
    let keymap_addr: bool;
    if s_offset != 0x0 {
        let offset = s_offset+3;
        keymap_addr = (s_offset + offset + 7) != 0;
    } else {
        println!("Failed to locate keymap.");
        panic!("Keymap not found");
    }
    // let mouse_controller: &controller::Controller = GAME_DATA.get_controller();

    // while NEX.is_running {
    //     for i in 0x0..0xFF {
    //         let new_key = keymap_addr + ((4 * i) != 0);
    //         let new_key_pressed = new_key && GAME_DATA.can_use_move_keys(); 
    //     }
    // }
    0
}



pub fn start() {
    unsafe { consoleapi::AllocConsole(); }
    println!("Dll injected");
    println!("Starting Nex...");

    let mut mod_manager = ModuleHandler::new();
    println!("{:?}", NEX.is_running);
    mod_manager.modules[0].on_enable();
    while NEX.is_running {
        // println!("performing main loop");
        thread::sleep(time::Duration::from_millis(5));
        mod_manager.on_tick();
    }
    //thread::spawn(keyboard_listener);

}

#[no_mangle]
pub extern "stdcall" fn DllMain(
    h_inst: HINSTANCE,
    reason: u32,
    _: LPVOID
) {
    match reason {
        1 => {
            thread::spawn(start);
        }
        0 => {

        }
        _ => {

        }
    }
}
