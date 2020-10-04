//! XDL: An extension layer to Rust-SDL2
//!
//! Port of FNA input module + Virtual input.
//!
//! Intended for games with orghogonal grid maps.
//!
//! # Getting started
//!
//! Create [`Input`] with raw SDL window and manage their lifecycle.
//!
//! See [`vi`] module for virtual input.
//!
//! # WIP
//!
//! * WIP: keyboard input (key repeat?)
//! * WIP: mouse input
//! * not started: touch input

pub mod axis;
pub mod utils;
pub mod vi;

mod keyboard;
mod mouse;

pub use num_enum;
pub use sdl2;

use sdl2::event::Event;

pub use keyboard::{Key, Keyboard};
pub use mouse::{Mouse, MouseInput};

/// All of the input state
#[derive(Debug, Clone)]
pub struct Input {
    pub kbd: Keyboard,
    pub mouse: Mouse,
}

impl Input {
    pub fn new(win: *mut sdl2::sys::SDL_Window) -> Self {
        Self {
            kbd: Keyboard::default(),
            mouse: Mouse::new(win),
        }
    }

    pub fn kbd(&self) -> &Keyboard {
        &self.kbd
    }

    pub fn mouse(&self) -> &Mouse {
        &self.mouse
    }
}

/// Lifecycle
impl Input {
    pub fn event(&mut self, ev: &Event) {
        self.kbd.event(ev);
        self.mouse.event(ev);
    }

    pub fn update(&mut self) {
        self.mouse.update();
    }

    pub fn on_end_frame(&mut self) {
        self.kbd.on_end_frame();
        self.mouse.on_end_frame();
    }
}
