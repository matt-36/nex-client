pub mod modules;
pub mod module_handler;
pub struct Nex {
    pub is_running: bool
}

impl Nex {
    pub const fn new() -> Self {
        Self {
            is_running: true
        }
    }
}