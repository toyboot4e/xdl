//! rokol backend

use crate::input::{
    keyboard::{Key, Keyboard},
    Input,
};
use std::collections::HashMap;

pub type ExternalKey = rokol::app::Key;
pub type Event = rokol::app::Event;

pub fn key_translation() -> HashMap<rokol::app::Key, Key> {
    use rokol::app::Key as RKey;

    [
        (RKey::A, Key::A),
        (RKey::B, Key::B),
        (RKey::C, Key::C),
        (RKey::D, Key::D),
        (RKey::E, Key::E),
        (RKey::F, Key::F),
        (RKey::G, Key::G),
        (RKey::H, Key::H),
        (RKey::I, Key::I),
        (RKey::J, Key::J),
        (RKey::K, Key::K),
        (RKey::L, Key::L),
        (RKey::M, Key::M),
        (RKey::N, Key::N),
        (RKey::O, Key::O),
        (RKey::P, Key::P),
        (RKey::Q, Key::Q),
        (RKey::R, Key::R),
        (RKey::S, Key::S),
        (RKey::T, Key::T),
        (RKey::U, Key::U),
        (RKey::V, Key::V),
        (RKey::W, Key::W),
        (RKey::X, Key::X),
        (RKey::Y, Key::Y),
        (RKey::Z, Key::Z),
        (RKey::Kbd0, Key::D0),
        (RKey::Kbd1, Key::D1),
        (RKey::Kbd2, Key::D2),
        (RKey::Kbd3, Key::D3),
        (RKey::Kbd4, Key::D4),
        (RKey::Kbd5, Key::D5),
        (RKey::Kbd6, Key::D6),
        (RKey::Kbd7, Key::D7),
        (RKey::Kbd8, Key::D8),
        (RKey::Kbd9, Key::D9),
        (RKey::KP0, Key::NumPad0),
        (RKey::KP1, Key::NumPad1),
        (RKey::KP2, Key::NumPad2),
        (RKey::KP3, Key::NumPad3),
        (RKey::KP4, Key::NumPad4),
        (RKey::KP5, Key::NumPad5),
        (RKey::KP6, Key::NumPad6),
        (RKey::KP7, Key::NumPad7),
        (RKey::KP8, Key::NumPad8),
        (RKey::KP9, Key::NumPad9),
        // (RKey::KPClear, Key::OemClear),
        (RKey::KPDecimal, Key::Decimal),
        (RKey::KPDivide, Key::Divide),
        (RKey::KPEnter, Key::Enter),
        (RKey::KPSubtract, Key::Subtract),
        (RKey::KPMultiply, Key::Multiply),
        (RKey::KPDecimal, Key::OemPeriod),
        (RKey::KPAdd, Key::Add),
        (RKey::F1, Key::F1),
        (RKey::F2, Key::F2),
        (RKey::F3, Key::F3),
        (RKey::F4, Key::F4),
        (RKey::F5, Key::F5),
        (RKey::F6, Key::F6),
        (RKey::F7, Key::F7),
        (RKey::F8, Key::F8),
        (RKey::F9, Key::F9),
        (RKey::F10, Key::F10),
        (RKey::F11, Key::F11),
        (RKey::F12, Key::F12),
        (RKey::F13, Key::F13),
        (RKey::F14, Key::F14),
        (RKey::F15, Key::F15),
        (RKey::F16, Key::F16),
        (RKey::F17, Key::F17),
        (RKey::F18, Key::F18),
        (RKey::F19, Key::F19),
        (RKey::F20, Key::F20),
        (RKey::F21, Key::F21),
        (RKey::F22, Key::F22),
        (RKey::F23, Key::F23),
        (RKey::F24, Key::F24),
        (RKey::Space, Key::Space),
        (RKey::Up, Key::Up),
        (RKey::Down, Key::Down),
        (RKey::Left, Key::Left),
        (RKey::Right, Key::Right),
        (RKey::LeftAlt, Key::LAlt),
        (RKey::RightAlt, Key::RAlt),
        (RKey::LeftControl, Key::LCtrl),
        (RKey::RightControl, Key::RCtrl),
        (RKey::LeftSuper, Key::LMeta),
        (RKey::RightSuper, Key::RMeta),
        (RKey::LeftShift, Key::LShift),
        (RKey::RightShift, Key::RShift),
        // (RKey::Application, Key::Apps),
        (RKey::Slash, Key::OemQuestion),
        (RKey::Backslash, Key::OemBackslash),
        (RKey::LeftBracket, Key::OemOpenBrackets),
        (RKey::RightBracket, Key::OemCloseBrackets),
        (RKey::CapsLock, Key::CapsLock),
        (RKey::Comma, Key::OemComma),
        (RKey::Delete, Key::Delete),
        (RKey::End, Key::End),
        (RKey::Backspace, Key::Back),
        (RKey::Enter, Key::Enter),
        (RKey::Escape, Key::Escape),
        (RKey::Home, Key::Home),
        (RKey::Insert, Key::Insert),
        (RKey::Minus, Key::OemMinus),
        (RKey::NumLock, Key::NumLock),
        (RKey::PageUp, Key::PageUp),
        (RKey::PageDown, Key::PageDown),
        (RKey::Pause, Key::Pause),
        (RKey::Period, Key::OemPeriod),
        // (RKey::Equals, Key::OemPlus),
        (RKey::PrintScreen, Key::PrintScreen),
        // (RKey::Quote, Key::OemQuotes),
        (RKey::ScrollLock, Key::Scroll),
        (RKey::Semicolon, Key::OemSemicolon),
        // (RKey::Sleep, Key::Sleep),
        (RKey::Tab, Key::Tab),
        // (RKey::Backquote, Key::OemTilde),
        // (RKey::VolumeUp, Key::VolumeUp),
        // (RKey::VolumeDown, Key::VolumeDown),
    ]
    .iter()
    .cloned()
    .collect()
}

/// Lifecycle
impl Keyboard {
    pub fn event(&mut self, ev: &Event) {
        use rokol::app::{EventType, Key};

        let ev_type = EventType::from_u32(ev.type_).unwrap();
        match ev_type {
            EventType::KeyDown => {
                let key = Key::from_u32(ev.key_code).unwrap();
                self.on_key_down(key);
            }
            EventType::KeyUp => {
                let key = Key::from_u32(ev.key_code).unwrap();
                self.on_key_up(key);
            }
            _ => {
                //
            }
        }
    }
}

impl Input {
    /// Event pump
    pub fn event(&mut self, ev: &rokol::app::Event) {
        self.kbd.event(ev);
        // self.mouse.event(ev);
    }

    pub fn on_end_frame(&mut self) {
        // swap buffers
        self.kbd.on_end_frame();
        // self.mouse.on_end_frame();
    }
}
