pub struct Controller {
    pad_0x0000: [u8; 0x50],
    pub left_click_down: bool,
    pub right_click_down: bool,
    pub wheel_down: bool,
    pub mouse_4_down: bool,
    pub mouse_5_down: bool,
}