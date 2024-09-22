use std::char;

pub type ScreenBuffer = [[char; 80]; 24];
pub struct Screen {
    screen_buf0: ScreenBuffer,
    screen_buf1: ScreenBuffer,
}

impl Screen {
    fn init(self) {
    }
}
