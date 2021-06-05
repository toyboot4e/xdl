//! Re-exported to super module

#![allow(dead_code)]

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use {
    num_enum::TryFromPrimitive,
    std::{collections::HashMap, convert::TryFrom},
};

use crate::{platform::ExternalKey, utils::Double};

/// XDL keycode
///
/// Can be created from supported backend's keycode (`ExternalKey`)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive)]
#[repr(u32)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Key {
    /// TODO: delete?
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
    LMeta = 91,
    RMeta = 92,
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
    Star = 106,
    Plus = 107,
    // Separator = 108, // ?
    Minus = 109,
    Decimal = 110,
    Slash = 111,
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
    LShift = 160,
    RShift = 161,
    LCtrl = 162,
    RCtrl = 163,
    LAlt = 164,
    RAlt = 165,
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

impl Key {
    /// a-z or symbol
    pub fn from_char(c: char) -> Option<Self> {
        let key = match c {
            'a' => Key::A,
            'b' => Key::B,
            'c' => Key::C,
            'd' => Key::D,
            'e' => Key::E,
            'f' => Key::F,
            'g' => Key::G,
            'h' => Key::H,
            'i' => Key::I,
            'j' => Key::J,
            'k' => Key::K,
            'l' => Key::L,
            'm' => Key::M,
            'n' => Key::N,
            'o' => Key::O,
            'p' => Key::P,
            'q' => Key::Q,
            'r' => Key::R,
            's' => Key::S,
            't' => Key::T,
            'u' => Key::U,
            'v' => Key::V,
            'w' => Key::W,
            'x' => Key::X,
            'y' => Key::Y,
            'z' => Key::Z,
            //
            ' ' => Key::Space,
            '+' => Key::Plus,
            '^' => Key::Minus,
            '*' => Key::Star,
            '/' => Key::Slash,
            '~' => Key::OemTilde,
            '.' => Key::OemPeriod,
            '?' => Key::OemQuestion,
            ';' => Key::OemSemicolon,
            '|' => Key::OemPipe,
            '{' => Key::OemOpenBrackets,
            '}' => Key::OemCloseBrackets,
            // TODO:
            // ':' => Key::Colon,
            // '\\' => Key::OemBackSlash,
            // '(' => Key::OpenParen,
            // ')' => Key::OpenParen,
            // '=' => Key::Equal,
            // '!' => Key::Bang,
            _ => return None,
        };
        Some(key)
    }
}

/// All of the keyboard states
#[derive(Debug, Clone)]
pub struct Keyboard {
    /// External keycode to XDL keycode
    e2x: HashMap<ExternalKey, Key>,
    pub(crate) states: Double<KeyboardStateSnapshot>,
}

impl Default for Keyboard {
    fn default() -> Self {
        Self {
            e2x: crate::platform::key_translation(),
            states: Double::default(),
        }
    }
}

impl Keyboard {
    pub fn on_end_frame(&mut self) {
        self.states.b.bits = self.states.a.bits;
    }

    /// Used to implement platform event listening function
    pub(crate) fn on_key_down(&mut self, external_key: ExternalKey) {
        let xdl_key = match self.e2x.get(&external_key) {
            Some(key) => key.clone(),
            None => return,
        };

        self.states.a.on_key_down(xdl_key);
    }

    /// Used to implement platform event listening function
    pub(crate) fn on_key_up(&mut self, external_key: ExternalKey) {
        let xdl_key = match self.e2x.get(&external_key) {
            Some(key) => key.clone(),
            None => return,
        };

        self.states.a.on_key_up(xdl_key);
    }
}

impl Keyboard {
    pub fn clear(&mut self) {
        self.states.a = KeyboardStateSnapshot { bits: [0; 8] };
        self.states.b = KeyboardStateSnapshot { bits: [0; 8] };
    }
}

/// Single key
impl Keyboard {
    pub fn is_key_down(&self, key: Key) -> bool {
        self.states.a.is_down(key)
    }

    pub fn is_key_up(&self, key: Key) -> bool {
        self.states.a.is_up(key)
    }

    pub fn is_key_pressed(&self, key: Key) -> bool {
        self.states.b.is_up(key) && self.states.a.is_down(key)
    }

    pub fn is_key_released(&self, key: Key) -> bool {
        self.states.b.is_down(key) && self.states.a.is_up(key)
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
