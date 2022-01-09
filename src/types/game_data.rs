

use std::{ffi::c_void, ptr::null};

use super::*;

pub struct GameData {
    client_instance: Option<client_instance::ClientInstance>,
    local_player: Option<local_player::LocalPlayer>,
    game_mode: Option<game_mode::GameMode>,
    controller: Option<controller::Controller>,
    raknet_instance: Option<raknet_instance::RaknetInstance>,

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
            local_player: None,
            game_mode: None,
            controller: None,
            raknet_instance: None,
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

    pub fn entity_list_tick(&mut self, entities: Vec<entity::Entity>) {

    }

    pub fn get_gui_data(&self) {
        
    }

    pub fn get_dll_instance(&self) -> i8 {
        return self.h_dll_instance
    }

    pub fn get_local_player(&self) -> Option<&local_player::LocalPlayer> {
        if !self.local_player.is_none() {
            return self.local_player.as_ref()
        } else {
            return None
        }
    }

    pub fn in_game(&self) -> bool {
        return self.local_player.is_none()
    }

    pub fn get_game_mode(&self) -> &game_mode::GameMode {
        return self.game_mode.as_ref().unwrap()
    }

    pub fn get_controller(&self) -> Option<&controller::Controller> {
        if !self.controller.is_none() {
            return self.controller.as_ref()
        }
        return None
    }
}