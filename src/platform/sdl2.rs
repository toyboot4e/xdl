//! Rust-SDL2 backend

use crate::input::{
    keyboard::{Key, Keyboard},
    Input,
};
use std::collections::HashMap;

pub type ExternalKey = sdl2::keyboard::Keycode;
pub type Event = sdl2::event::Event;

/// Lifecycle
impl Keyboard {
    pub fn event(&mut self, ev: &Event) {
        use sdl2::event::Event;
        match ev {
            Event::KeyDown {
                keycode: Some(sdl_key),
                ..
            } => {
                self.on_key_down(*sdl_key);
            }
            Event::KeyUp {
                keycode: Some(sdl_key),
                ..
            } => {
                self.on_key_up(*sdl_key);
            }
            _ => {}
        }
    }
}

impl Input {
    pub fn from_window(_win: *mut sdl2::sys::SDL_Window) -> Self {
        Self {
            kbd: Keyboard::default(),
            // mouse: Mouse::new(win),
        }
    }

    pub fn event(&mut self, ev: &sdl2::event::Event) {
        self.kbd.event(ev);
        // self.mouse.event(ev);
    }

    pub fn on_end_frame(&mut self) {
        // swap buffers
        self.kbd.on_end_frame();
        // self.mouse.on_end_frame();
    }
}

pub fn key_translation() -> HashMap<sdl2::keyboard::Keycode, Key> {
    pub use sdl2::{
        event::Event,
        keyboard::{Keycode, Mod, Scancode},
    };

    [
        (Keycode::A, Key::A),
        (Keycode::B, Key::B),
        (Keycode::C, Key::C),
        (Keycode::D, Key::D),
        (Keycode::E, Key::E),
        (Keycode::F, Key::F),
        (Keycode::G, Key::G),
        (Keycode::H, Key::H),
        (Keycode::I, Key::I),
        (Keycode::J, Key::J),
        (Keycode::K, Key::K),
        (Keycode::L, Key::L),
        (Keycode::M, Key::M),
        (Keycode::N, Key::N),
        (Keycode::O, Key::O),
        (Keycode::P, Key::P),
        (Keycode::Q, Key::Q),
        (Keycode::R, Key::R),
        (Keycode::S, Key::S),
        (Keycode::T, Key::T),
        (Keycode::U, Key::U),
        (Keycode::V, Key::V),
        (Keycode::W, Key::W),
        (Keycode::X, Key::X),
        (Keycode::Y, Key::Y),
        (Keycode::Z, Key::Z),
        (Keycode::Num0, Key::D0),
        (Keycode::Num1, Key::D1),
        (Keycode::Num2, Key::D2),
        (Keycode::Num3, Key::D3),
        (Keycode::Num4, Key::D4),
        (Keycode::Num5, Key::D5),
        (Keycode::Num6, Key::D6),
        (Keycode::Num7, Key::D7),
        (Keycode::Num8, Key::D8),
        (Keycode::Num9, Key::D9),
        (Keycode::Kp0, Key::NumPad0),
        (Keycode::Kp1, Key::NumPad1),
        (Keycode::Kp2, Key::NumPad2),
        (Keycode::Kp3, Key::NumPad3),
        (Keycode::Kp4, Key::NumPad4),
        (Keycode::Kp5, Key::NumPad5),
        (Keycode::Kp6, Key::NumPad6),
        (Keycode::Kp7, Key::NumPad7),
        (Keycode::Kp8, Key::NumPad8),
        (Keycode::Kp9, Key::NumPad9),
        (Keycode::KpClear, Key::OemClear),
        (Keycode::KpDecimal, Key::Decimal),
        (Keycode::KpDivide, Key::Slash),
        (Keycode::KpEnter, Key::Enter),
        (Keycode::KpMinus, Key::Minus),
        (Keycode::KpMultiply, Key::Star),
        (Keycode::KpPeriod, Key::OemPeriod),
        (Keycode::KpPlus, Key::Plus),
        (Keycode::F1, Key::F1),
        (Keycode::F2, Key::F2),
        (Keycode::F3, Key::F3),
        (Keycode::F4, Key::F4),
        (Keycode::F5, Key::F5),
        (Keycode::F6, Key::F6),
        (Keycode::F7, Key::F7),
        (Keycode::F8, Key::F8),
        (Keycode::F9, Key::F9),
        (Keycode::F10, Key::F10),
        (Keycode::F11, Key::F11),
        (Keycode::F12, Key::F12),
        (Keycode::F13, Key::F13),
        (Keycode::F14, Key::F14),
        (Keycode::F15, Key::F15),
        (Keycode::F16, Key::F16),
        (Keycode::F17, Key::F17),
        (Keycode::F18, Key::F18),
        (Keycode::F19, Key::F19),
        (Keycode::F20, Key::F20),
        (Keycode::F21, Key::F21),
        (Keycode::F22, Key::F22),
        (Keycode::F23, Key::F23),
        (Keycode::F24, Key::F24),
        (Keycode::Space, Key::Space),
        (Keycode::Up, Key::Up),
        (Keycode::Down, Key::Down),
        (Keycode::Left, Key::Left),
        (Keycode::Right, Key::Right),
        (Keycode::LAlt, Key::LAlt),
        (Keycode::RAlt, Key::RAlt),
        (Keycode::LCtrl, Key::LCtrl),
        (Keycode::RCtrl, Key::RCtrl),
        (Keycode::LGui, Key::LMeta),
        (Keycode::RGui, Key::RMeta),
        (Keycode::LShift, Key::LShift),
        (Keycode::RShift, Key::RShift),
        (Keycode::Application, Key::Apps),
        (Keycode::Slash, Key::OemQuestion),
        (Keycode::Backslash, Key::OemBackslash),
        (Keycode::LeftBracket, Key::OemOpenBrackets),
        (Keycode::RightBracket, Key::OemCloseBrackets),
        (Keycode::CapsLock, Key::CapsLock),
        (Keycode::Comma, Key::OemComma),
        (Keycode::Delete, Key::Delete),
        (Keycode::End, Key::End),
        (Keycode::Backspace, Key::Back),
        (Keycode::Return, Key::Enter),
        (Keycode::Escape, Key::Escape),
        (Keycode::Home, Key::Home),
        (Keycode::Insert, Key::Insert),
        (Keycode::Minus, Key::OemMinus),
        (Keycode::NumLockClear, Key::NumLock),
        (Keycode::PageUp, Key::PageUp),
        (Keycode::PageDown, Key::PageDown),
        (Keycode::Pause, Key::Pause),
        (Keycode::Period, Key::OemPeriod),
        // FIXME:
        (Keycode::Equals, Key::OemPlus),
        (Keycode::PrintScreen, Key::PrintScreen),
        (Keycode::Quote, Key::OemQuotes),
        (Keycode::ScrollLock, Key::Scroll),
        (Keycode::Semicolon, Key::OemSemicolon),
        (Keycode::Sleep, Key::Sleep),
        (Keycode::Tab, Key::Tab),
        (Keycode::Backquote, Key::OemTilde),
        (Keycode::VolumeUp, Key::VolumeUp),
        (Keycode::VolumeDown, Key::VolumeDown),
    ]
    .iter()
    .cloned()
    .collect()
}
