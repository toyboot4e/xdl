#![allow(dead_code)]

pub mod keyboard;
// pub mod mouse;

use self::keyboard::Keyboard;

/// All of the input states
#[derive(Debug, Clone, Default)]
pub struct Input {
    pub kbd: Keyboard,
    // pub mouse: Mouse,
}

impl Input {
    pub fn new() -> Self {
        Self {
            kbd: Keyboard::default(),
        }
    }

    /// Resets all the states
    pub fn clear(&mut self) {
        self.kbd.clear();
        // self.mouse.clear();
    }
}
