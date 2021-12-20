

use std::{ffi::c_void};

use super::*;

pub struct GameData {
    client_instance: client_instance::ClientInstance,
    local_player: local_player::LocalPlayer,
    game_mode: game_mode::GameMode,
    controller: controller::Controller,
    raknet_instance: raknet_instance::RaknetInstance,

    h_dll_instance: c_void,

    chest_list: [()]
}

impl GameData {

}