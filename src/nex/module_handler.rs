use crate::nex::modules::air_speed::AirSpeed;

use super::modules::*;

pub trait Module {
    fn on_enable(&mut self) {
        
    }
    fn on_tick(&mut self) {
        
    }
}

pub struct ModuleHandler {
    pub modules: Vec<Box<dyn Module>>
}

impl ModuleHandler {
    pub fn new() -> Self {
        let modlist: Vec<Box<dyn Module>> = vec![
            // Air jump
            Box::new(air_jump::AirJump::new()),
            Box::new(AirSpeed::new())
            //
        ];
        println!("Created module manager");
        
        // Construct modulehandler
        Self {
            modules: modlist
        }
    }

    pub fn on_tick(&mut self) {
        for module in self.modules.iter_mut() {
            // println!("ticking {:?}", module.get_name());
            module.on_tick();
        }
    }
}