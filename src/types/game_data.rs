

use std::{ffi::{c_void, CString}, ptr::null};

use winapi::um::{libloaderapi::GetModuleHandleA, winnt::LPCSTR};

use crate::utils;

use super::{client_instance::ClientInstance, player::Player, controller::Controller, game_mode::GameMode, entity::Entity};

pub struct GameData {
    client_instance: Option<ClientInstance>,
    player: Option<Player>,
    game_mode: Option<GameMode>,
    controller: Option<Controller>,
    // raknet_instance: Option<RaknetInstance>,

    h_dll_instance: i8,

    chest_list: Vec<()>,

    pub fov: f32,
    pub fps: i16,
    pub frame_count: i32,
    pub cps_left: i8,
    pub cps_right: i8,
    pub left_click_count: i32,
    pub right_click_count: i32,
}

impl GameData {
    pub const fn new() -> Self {
        Self {
            client_instance: None,
            player: None,
            game_mode: None,
            controller: None,
            // raknet_instance: None,
            h_dll_instance: 0,
            chest_list: Vec::new(),
            fov: 0.0,
            fps: 0,
            frame_count: 0,
            cps_left: 0,
            cps_right: 0,
            left_click_count: 0,
            right_click_count: 0
        }
    }

    pub fn can_use_move_keys(&self) -> bool {
        // let mc = self.client_instance.minecraft_game;
        // if mc.is_none() {
        //     false
        // } else {
        //     mc.can_use_keybinds()
        // }
        true
    }

    pub fn is_key_down(&self, key: i8) {

    }

    pub fn is_key_pressed(&self, key: i8) {

    }

    pub fn is_right_click_down(&self) {

    }

    pub fn is_left_click_down(&self) {

    }

    pub fn is_wheel_down(&self) {

    }

    pub fn add_chest(&mut self) {

    }

    pub fn clear_chests(&mut self) {

    }

    pub fn entity_list_tick(&mut self, entities: Vec<Entity>) {

    }

    pub fn get_gui_data(&self) {
        
    }

    pub fn get_dll_instance(&self) -> i8 {
        return self.h_dll_instance
    }

    pub fn get_local_player(&self) -> Option<&Player> {
        if !self.player.is_none() {
            return self.player.as_ref()
        } else {
            return None
        }
    }

    pub fn in_game(&self) -> bool {
        return self.player.is_none()
    }

    pub fn get_game_mode(&self) -> &GameMode {
        return self.game_mode.as_ref().unwrap()
    }

    pub fn get_controller(&self) -> Option<&Controller> {
        if !self.controller.is_none() {
            return self.controller.as_ref()
        }
        return None
    }
    pub fn get_player(&self) -> Option<&Player> {
        if !self.player.is_none() {
            return self.player.as_ref()
        }
        return None
    }
    pub fn get_client_instance(&self) -> *const ClientInstance {
        let base;
        unsafe {
            base = GetModuleHandleA(CString::new("Minecraft.Windows.exe").unwrap().as_ptr() as LPCSTR) as u64;
        }
        println!("[DEBUG] Client instance: retrieved base");
        let offsets = vec![0x0, 0x50, 0x0];
        let client = utils::find_multi_level_pointer((base + 0x04223B60).try_into().unwrap(), offsets) as *const ClientInstance;
        println!("[DEBUG] Client instance: found client instance");
        client
    }
}