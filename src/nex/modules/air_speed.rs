use crate::{nex::module_handler::Module, types::actor::Actor, GAME_DATA};

pub struct AirSpeed {
    active: bool,
}

impl AirSpeed {
    pub fn new() -> Self {
        Self { active: false }
    }
}

impl Module for AirSpeed {
    fn on_enable(&mut self) {
        self.active = true;
    }
    fn on_tick(&mut self) {
        if self.active {
            unsafe {
                let client = &*GAME_DATA.get_client_instance();
                println!("on ground: {:?}", client.on_ground())
            }
        }
    }
}
