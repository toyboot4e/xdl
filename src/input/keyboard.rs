//! Re-exported to super module

// #![allow(dead_code)]

use std::{collections::HashMap, convert::TryFrom};

use num_enum::TryFromPrimitive;

pub use sdl2::{
    event::Event,
    keyboard::{Keycode, Mod, Scancode},
};

use crate::utils::Double;

/// XDL keycode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive)]
#[repr(u32)]
pub enum Key {
    None = 0,
    /// Backspace
    Back = 8,
    Tab = 9,
    Enter = 13,
    CapsLock = 20,
    Escape = 27,
    Space = 32,
    PageUp = 33,
    PageDown = 34,
    End = 35,
    Home = 36,
    Left = 37,
    Up = 38,
    Right = 39,
    Down = 40,
    Select = 41,
    Print = 42,
    Execute = 43,
    PrintScreen = 44,
    Insert = 45,
    Delete = 46,
    Help = 47,
    /// Digit 0
    D0 = 48,
    D1 = 49,
    D2 = 50,
    D3 = 51,
    D4 = 52,
    D5 = 53,
    D6 = 54,
    D7 = 55,
    D8 = 56,
    D9 = 57,
    A = 65,
    B = 66,
    C = 67,
    D = 68,
    E = 69,
    F = 70,
    G = 71,
    H = 72,
    I = 73,
    J = 74,
    K = 75,
    L = 76,
    M = 77,
    N = 78,
    O = 79,
    P = 80,
    Q = 81,
    R = 82,
    S = 83,
    T = 84,
    U = 85,
    V = 86,
    W = 87,
    X = 88,
    Y = 89,
    Z = 90,
    LeftWindows = 91,
    RightWindows = 92,
    Apps = 93,
    Sleep = 95,
    NumPad0 = 96,
    NumPad1 = 97,
    NumPad2 = 98,
    NumPad3 = 99,
    NumPad4 = 100,
    NumPad5 = 101,
    NumPad6 = 102,
    NumPad7 = 103,
    NumPad8 = 104,
    NumPad9 = 105,
    Multiply = 106,
    Add = 107,
    Separator = 108,
    Subtract = 109,
    Decimal = 110,
    Divide = 111,
    F1 = 112,
    F2 = 113,
    F3 = 114,
    F4 = 115,
    F5 = 116,
    F6 = 117,
    F7 = 118,
    F8 = 119,
    F9 = 120,
    F10 = 121,
    F11 = 122,
    F12 = 123,
    F13 = 124,
    F14 = 125,
    F15 = 126,
    F16 = 127,
    F17 = 128,
    F18 = 129,
    F19 = 130,
    F20 = 131,
    F21 = 132,
    F22 = 133,
    F23 = 134,
    F24 = 135,
    NumLock = 144,
    Scroll = 145,
    LeftShift = 160,
    RightShift = 161,
    LeftControl = 162,
    RightControl = 163,
    LeftAlt = 164,
    RightAlt = 165,
    BrowserBack = 166,
    BrowserForward = 167,
    BrowserRefresh = 168,
    BrowserStop = 169,
    BrowserSearch = 170,
    BrowserFavorites = 171,
    BrowserHome = 172,
    VolumeMute = 173,
    VolumeDown = 174,
    VolumeUp = 175,
    MediaNextTrack = 176,
    MediaPreviousTrack = 177,
    MediaStop = 178,
    MediaPlayPause = 179,
    LaunchMail = 180,
    SelectMedia = 181,
    LaunchApplication1 = 182,
    LaunchApplication2 = 183,
    /// The OEM Semicolon key on a US standard keyboard.
    OemSemicolon = 186,
    OemPlus = 187,
    OemComma = 188,
    OemMinus = 189,
    OemPeriod = 190,
    OemQuestion = 191,
    OemTilde = 192,
    OemOpenBrackets = 219,
    OemPipe = 220,
    OemCloseBrackets = 221,
    OemQuotes = 222,
    Oem8 = 223,
    OemBackslash = 226,
    ProcessKey = 229,
    Attn = 246,
    Crsel = 247,
    Exsel = 248,
    EraseEof = 249,
    Play = 250,
    Zoom = 251,
    Pa1 = 253,
    OemClear = 254,
    ChatPadGreen = 0xCA,
    ChatPadOrange = 0xCB,
    Pause = 0x13,
    ImeConvert = 0x1c,
    ImeNoConvert = 0x1d,
    Kana = 0x15,
    Kanji = 0x19,
    OemAuto = 0xf3,
    OemCopy = 0xf2,
    /// OEM Enlarge Window key.
    OemEnlW = 0xf4,
}

/// All of the mouse states
#[derive(Debug, Clone)]
pub struct Keyboard {
    /// SDL2 keycode to XDL keycode
    s2f: HashMap<Keycode, Key>,
    kbd: Double<KeyboardStateSnapshot>,
}

impl Default for Keyboard {
    fn default() -> Self {
        Self {
            s2f: self::gen_key_translation(),
            kbd: Double::default(),
        }
    }
}

/// Single key
impl Keyboard {
    pub fn is_key_down(&self, key: Key) -> bool {
        self.kbd.a.is_down(key)
    }

    pub fn is_key_up(&self, key: Key) -> bool {
        self.kbd.a.is_up(key)
    }

    pub fn is_key_pressed(&self, key: Key) -> bool {
        self.kbd.b.is_up(key) && self.kbd.a.is_down(key)
    }

    pub fn is_key_released(&self, key: Key) -> bool {
        self.kbd.b.is_down(key) && self.kbd.a.is_up(key)
    }
}

/// Multiple keys
impl Keyboard {
    pub fn is_any_key_down<'a>(&self, keys: impl IntoIterator<Item = &'a Key>) -> bool {
        keys.into_iter().any(|key| self.is_key_down(*key))
    }

    pub fn is_any_key_up<'a>(&self, keys: impl IntoIterator<Item = &'a Key>) -> bool {
        keys.into_iter().any(|key| self.is_key_up(*key))
    }

    pub fn is_any_key_pressed<'a>(&self, keys: impl IntoIterator<Item = &'a Key>) -> bool {
        keys.into_iter().any(|key| self.is_key_pressed(*key))
    }

    pub fn is_any_key_released<'a>(&self, keys: impl IntoIterator<Item = &'a Key>) -> bool {
        keys.into_iter().any(|key| self.is_key_released(*key))
    }
}

/// Lifecycle
impl Keyboard {
    pub fn event(&mut self, ev: &Event) {
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

    pub fn on_end_frame(&mut self) {
        self.kbd.b.bits = self.kbd.a.bits;
    }
}

impl Keyboard {
    fn on_key_down(&mut self, sdl_key: Keycode) {
        let xdl_key = match self.s2f.get(&sdl_key) {
            Some(key) => key.clone(),
            None => return,
        };
        self.kbd.a.on_key_down(xdl_key);
    }

    fn on_key_up(&mut self, sdl_key: Keycode) {
        let xdl_key = match self.s2f.get(&sdl_key) {
            Some(key) => key.clone(),
            None => return,
        };
        self.kbd.a.on_key_up(xdl_key);
    }
}

/// 256 bits for key states (up or down)
///
/// Compare two snapshots to see if the key is pressed or released.
///
/// Based on: http://graphics.stxdlord.edu/~seander/bithacks.html#CountBitsSetParallel
#[derive(Debug, Clone, Default)]
pub struct KeyboardStateSnapshot {
    pub bits: [u32; 8],
}

impl KeyboardStateSnapshot {
    // fn from_keys(akeys: &[Keycode]) -> Self {}

    pub fn on_key_down(&mut self, key: Key) {
        let mask = 1 << ((key as u32) & 0x1f);
        let ix = key as usize >> 5;
        self.bits[ix] |= mask;
    }

    pub fn on_key_up(&mut self, key: Key) {
        let mask = 1 << ((key as u32) & 0x1f);
        let ix = key as usize >> 5;
        self.bits[ix] &= !mask;
    }

    pub fn is_down(&self, key: Key) -> bool {
        let mask: u32 = 1 << ((key as u32) & 0x1f);
        let ix = key as usize >> 5;
        (self.bits[ix] & mask) != 0
    }

    pub fn is_up(&self, key: Key) -> bool {
        !self.is_down(key)
    }

    pub fn pressed_keys(&self) -> Vec<Key> {
        let count = self
            .bits
            .iter()
            .map(|bits| Self::count_bits(*bits) as usize)
            .sum();

        if count == 0 {
            return Vec::new();
        }

        let mut keys = Vec::with_capacity(count);

        let mut ix = 0;
        for bits in self.bits.iter() {
            if *bits != 0 {
                ix = Self::store_keys(*bits, 0 * 32, &mut keys, ix);
            }
        }

        keys
    }
}

impl KeyboardStateSnapshot {
    /// http://graphics.stxdlord.edu/~seander/bithacks.html#CountBitsSetParallel
    fn count_bits(key: u32) -> u32 {
        let mut v = key as u32;
        v = v - ((v >> 1) & 0x55555555);
        v = (v & 0x33333333) + ((v >> 2) & 0x33333333);
        ((v + (v >> 4) & 0xF0F0F0F) * 0x1010101) >> 24
    }

    fn store_keys(keys: u32, offset: u32, pressed_keys: &mut [Key], mut ix: usize) -> usize {
        for i in 0..32 {
            if (keys & (1 << i)) != 0 {
                pressed_keys[ix] = Key::try_from(offset + i).unwrap();
                ix += 1;
            }
        }

        ix
    }
}

pub fn gen_key_translation() -> HashMap<Keycode, Key> {
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
        (Keycode::KpDivide, Key::Divide),
        (Keycode::KpEnter, Key::Enter),
        (Keycode::KpMinus, Key::Subtract),
        (Keycode::KpMultiply, Key::Multiply),
        (Keycode::KpPeriod, Key::OemPeriod),
        (Keycode::KpPlus, Key::Add),
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
        (Keycode::LAlt, Key::LeftAlt),
        (Keycode::RAlt, Key::RightAlt),
        (Keycode::LCtrl, Key::LeftControl),
        (Keycode::RCtrl, Key::RightControl),
        (Keycode::LGui, Key::LeftWindows),
        (Keycode::RGui, Key::RightWindows),
        (Keycode::LShift, Key::LeftShift),
        (Keycode::RShift, Key::RightShift),
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
