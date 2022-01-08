/*!
Virtual input, bundles of input states

# About

Virtual input is good for typical input abstraction. For example, your "select key" may be any
of enter, space, a gamepad button or even left click. Then the virtual input is perfect for
bundling them.

However, they are not generic enough. For example, you might want to handle left click in a
different way from enter key. Then you have to build your custom input system like UI commands,
maybe using virtual input.

# Lifecycle

Lifecycle types need to be `update`d when you update your game.

# Coordinate system

X axis goes from left to right. Y axis goes from up to down. If it doesn't match your needs.. sorry!

# Priority

Latest inputs always come as current state.

# serde support

TODO: add RON examples
*/

use std::time::Duration;

use crate::{
    axis::{Dir4, Dir8, Sign},
    Input, Key,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Key repeat settings
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(untagged))]
pub enum KeyRepeatConfig {
    Repeat { first: Duration, multi: Duration },
    NoRepeat,
}

impl Default for KeyRepeatConfig {
    fn default() -> Self {
        Self::NoRepeat
    }
}

/// Constructors
impl KeyRepeatConfig {
    pub fn repeat(first: Duration, multi: Duration) -> Self {
        KeyRepeatConfig::Repeat { first, multi }
    }

    pub fn no_repeat() -> Self {
        KeyRepeatConfig::NoRepeat
    }
}

// --------------------------------------------------------------------------------
// State

/// Down | Up | Pressed | Released
///
/// Repeats are not considered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RawButtonState {
    Down,
    Up,
    Pressed,
    Released,
}

#[derive(Debug, Clone, Default)]
struct KeyRepeatState {
    /// Key repeat configuration
    config: KeyRepeatConfig,
    /// Loops when it repeats
    accum_repeat: Duration,
    /// Does not loop
    accum_down: Duration,
    /// True until first repeat
    is_on_first_repeat: bool,
}

impl KeyRepeatState {
    pub fn new(repeat: KeyRepeatConfig) -> Self {
        Self {
            config: repeat,
            accum_repeat: Duration::new(0, 0),
            accum_down: Duration::new(0, 0),
            is_on_first_repeat: false,
        }
    }
}

/// Lifecycle
impl KeyRepeatState {
    /// Returns if it's repeating or not
    fn update(&mut self, state: RawButtonState, dt: Duration) -> bool {
        match state {
            RawButtonState::Up | RawButtonState::Released => {
                self.accum_repeat = Duration::new(0, 0);
                self.accum_down = Duration::new(0, 0);
                self.is_on_first_repeat = false;
                false
            }
            RawButtonState::Pressed => {
                self.accum_repeat = Duration::new(0, 0);
                self.accum_down = Duration::new(0, 0);
                self.is_on_first_repeat = true;
                false
            }
            // Down state may be repeating
            RawButtonState::Down => {
                let repeat_duration = match self.config {
                    KeyRepeatConfig::NoRepeat => return false,
                    KeyRepeatConfig::Repeat { first, multi } => {
                        if self.is_on_first_repeat {
                            first
                        } else {
                            multi
                        }
                    }
                };

                self.accum_repeat += dt;
                self.accum_down += dt;

                let mut is_repeating = false;

                // basically it's just an if branch but in case too long time passed
                while self.accum_repeat > repeat_duration {
                    is_repeating = true;
                    self.is_on_first_repeat = false;
                    self.accum_repeat -= repeat_duration;
                }

                is_repeating
            }
        }
    }
}

/// [`Key`] with optionally modifier keys
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct KeyEntry {
    key: Key,
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "is_false"))]
    ctrl: bool,
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "is_false"))]
    shift: bool,
    #[cfg_attr(feature = "serde", serde(default, skip_serializing_if = "is_false"))]
    meta: bool,
}

#[cfg(feature = "serde")]
fn is_false(b: &bool) -> bool {
    *b == false
}

impl From<Key> for KeyEntry {
    fn from(key: Key) -> KeyEntry {
        Self {
            key,
            ctrl: false,
            shift: false,
            meta: false,
        }
    }
}

impl KeyEntry {
    pub fn key(key: Key) -> Self {
        Self {
            key,
            ctrl: false,
            shift: false,
            meta: false,
        }
    }
}

/// Set of any kind of inputs
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct InputBundle {
    pub keys: Vec<KeyEntry>,
    // pub mouse: Vec<MouseInput>,
}

impl InputBundle {
    fn state(&self, input: &Input) -> RawButtonState {
        let mut is_any_down = false;
        let mut is_any_released = false;

        for entry in self.keys.iter() {
            let mut is_pressed = true;
            let mut is_down = true;
            let mut is_down_prev = true;

            macro_rules! _add {
                ($($key:expr),+ $(,)?) => {
                    $(
                        is_pressed &= input.kbd.is_key_pressed($key);
                        is_down &= input.kbd.is_key_down($key);
                        is_down_prev |= input.kbd.states.b.is_down($key);
                    )+
                };
            }

            _add!(entry.key);
            if entry.ctrl {
                _add!(Key::LCtrl, Key::RCtrl);
            }
            if entry.shift {
                _add!(Key::LShift, Key::RShift);
            }
            if entry.meta {
                _add!(Key::LMeta, Key::RMeta);
            }

            if is_pressed {
                return RawButtonState::Pressed;
            }
            is_any_down |= is_down;
            is_any_released |= is_down_prev && is_down;
        }

        // for m in self.mouse.iter().map(|m| m.clone()) {
        //     if input.mouse.is_pressed(m) {
        //         return RawButtonState::Pressed;
        //     }
        //     is_any_down |= input.mouse.is_down(m);
        //     is_any_released |= input.mouse.is_released(m);
        // }

        if is_any_down {
            RawButtonState::Down
        } else {
            if is_any_released {
                RawButtonState::Released
            } else {
                RawButtonState::Up
            }
        }
    }
}

/// Down | Up | Pressed | Repeating | Released
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum StrictButtonState {
    Down,
    Up,
    Pressed,
    Repeating,
    Released,
}

#[cfg(feature = "serde")]
pub mod button_serde_with {
    //! `serde` `Button` as [`InputBundle`]. NOTE: `KeyRepeatState` will be `skip`ped

    use super::*;
    use serde::{
        de::{Deserialize, Deserializer},
        ser::{Serialize, Serializer},
    };

    pub fn serialize<S>(value: &Button, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        value.input.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Button, D::Error>
    where
        D: Deserializer<'de>,
    {
        let input = InputBundle::deserialize(deserializer)?;
        Ok(Button::new(input, KeyRepeatConfig::NoRepeat))
    }
}

/// Input bundle with repeat state
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Button {
    pub input: InputBundle,
    pub state: StrictButtonState,
    #[cfg_attr(feature = "serde", serde(skip))]
    repeat: KeyRepeatState,
}

impl Button {
    pub fn new(bundle: InputBundle, repeat_cfg: KeyRepeatConfig) -> Self {
        Self {
            input: bundle,
            state: StrictButtonState::Up,
            repeat: KeyRepeatState::new(repeat_cfg),
        }
    }

    pub fn set_repeat_config(&mut self, cfg: KeyRepeatConfig) {
        self.repeat = KeyRepeatState::new(cfg);
    }

    pub fn is_down(&self) -> bool {
        matches!(
            self.state,
            StrictButtonState::Down | StrictButtonState::Pressed | StrictButtonState::Repeating
        )
    }

    pub fn is_pressed(&self) -> bool {
        matches!(
            self.state,
            StrictButtonState::Pressed | StrictButtonState::Repeating
        )
    }

    /// How long it's been down
    pub fn accum_down(&self) -> Duration {
        self.repeat.accum_down
    }
}

/// Lifecycle
impl Button {
    pub fn update(&mut self, input: &Input, dt: Duration) {
        let state = self.input.state(input);

        let is_repeating = self.repeat.update(state, dt);

        self.state = if is_repeating {
            StrictButtonState::Repeating
        } else {
            match state {
                RawButtonState::Down => StrictButtonState::Down,
                RawButtonState::Up => StrictButtonState::Up,
                RawButtonState::Pressed => StrictButtonState::Pressed,
                RawButtonState::Released => StrictButtonState::Released,
            }
        };
    }
}

// --------------------------------------------------------------------------------
// Higher-level buttons

/// Neg | Pos | Neutral
///
/// WARNING: It doesn't store key repeat configuration on `serde`.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AxisButton {
    /// Positive input
    #[cfg_attr(feature = "serde", serde(with = "button_serde_with"))]
    pub pos: Button,
    /// Negative input
    #[cfg_attr(feature = "serde", serde(with = "button_serde_with"))]
    pub neg: Button,
}

/// Lifecycle
impl AxisButton {
    pub fn update(&mut self, input: &Input, dt: Duration) {
        self.pos.update(input, dt);
        self.neg.update(input, dt);
    }
}

impl AxisButton {
    /// Selects down sign pressed lately
    pub fn sign_down(&self) -> Sign {
        match [self.pos.is_down(), self.neg.is_down()] {
            [true, true] => {
                if self.pos.repeat.accum_down <= self.neg.repeat.accum_down {
                    Sign::Pos
                } else {
                    Sign::Neg
                }
            }
            [true, false] => Sign::Pos,
            [false, true] => Sign::Neg,
            [false, false] => Sign::Neutral,
        }
    }

    /// Selects pressed sign pressed lately
    pub fn sign_pressed(&self) -> Sign {
        match [self.pos.is_down(), self.neg.is_down()] {
            [true, true] => {
                if self.pos.repeat.accum_down <= self.neg.repeat.accum_down {
                    if self.pos.is_pressed() {
                        Sign::Pos
                    } else {
                        Sign::Neutral
                    }
                } else {
                    if self.neg.is_pressed() {
                        Sign::Neg
                    } else {
                        Sign::Neutral
                    }
                }
            }
            [true, false] => {
                if self.pos.is_pressed() {
                    Sign::Pos
                } else {
                    Sign::Neutral
                }
            }
            [false, true] => {
                if self.neg.is_pressed() {
                    Sign::Neg
                } else {
                    Sign::Neutral
                }
            }
            [false, false] => Sign::Neutral,
        }
    }

    /// How long it's been down
    pub fn accum_down(&self) -> Duration {
        // select sign down lately
        std::cmp::min(self.pos.repeat.accum_down, self.neg.repeat.accum_down)
    }
}

/// [x, y] axes translated as direction
///
/// [x, y] components are "mixed" to make directions. For example, [1, 1] is interpreted as
/// south-east.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AxisDirButton {
    x: AxisButton,
    y: AxisButton,
}

impl AxisDirButton {
    /// Creates axis from [positive, negative] input bundle in (x, y) axis
    ///
    /// Makes sure that the key repeat configuration is shared among buttons (while the states are
    /// not shared).
    pub fn new(repeat: KeyRepeatConfig, xs: [InputBundle; 2], ys: [InputBundle; 2]) -> Self {
        let x_pos = Button::new(xs[0].clone(), repeat);
        let x_neg = Button::new(xs[1].clone(), repeat);
        let y_pos = Button::new(ys[0].clone(), repeat);
        let y_neg = Button::new(ys[1].clone(), repeat);

        Self {
            x: AxisButton {
                pos: x_pos,
                neg: x_neg,
            },
            y: AxisButton {
                pos: y_pos,
                neg: y_neg,
            },
        }
    }
}

/// Lifecycle
impl AxisDirButton {
    pub fn update(&mut self, input: &Input, dt: Duration) {
        self.x.update(input, dt);
        self.y.update(input, dt);
    }
}

impl AxisDirButton {
    /// Creates a directional output mixing axis inputs
    pub fn dir4_down(&self) -> Option<Dir4> {
        // mix down inputs (not pressed inputs)
        let x = self.x.sign_down().to_i8();
        let y = self.y.sign_down().to_i8();
        self.dir4(x, y)
    }

    /// Creates a directional output mixing axis inputs
    pub fn dir4_pressed(&self) -> Option<Dir4> {
        // mix down inputs (not pressed inputs)
        let x = self.x.sign_pressed().to_i8();
        let y = self.y.sign_pressed().to_i8();
        self.dir4(x, y)
    }

    /// Creates a directional output mixing axis inputs
    pub fn dir8_down(&self) -> Option<Dir8> {
        let x = self.x.sign_down().to_i8();
        let y = self.y.sign_down().to_i8();
        self.dir8(x, y)
    }

    /// Creates a directional output mixing axis inputs
    pub fn dir8_pressed(&self) -> Option<Dir8> {
        let x = self.x.sign_pressed().to_i8();
        let y = self.y.sign_pressed().to_i8();
        self.dir8(x, y)
    }

    fn dir4(&self, x: i32, y: i32) -> Option<Dir4> {
        Some(match [x, y] {
            [0, 0] => return None,
            // clockwise
            [0, -1] => Dir4::N,
            [1, 0] => Dir4::E,
            [0, 1] => Dir4::S,
            [-1, 0] => Dir4::W,
            [_, _] => {
                // select axis down lately
                if self.x.accum_down() <= self.y.accum_down() {
                    match x {
                        1 => Dir4::E,
                        -1 => Dir4::W,
                        _ => unreachable!(),
                    }
                } else {
                    match y {
                        1 => Dir4::S,
                        -1 => Dir4::N,
                        _ => unreachable!(),
                    }
                }
            }
        })
    }

    fn dir8(&self, x: i32, y: i32) -> Option<Dir8> {
        Some(match [x, y] {
            [0, 0] => return None,
            // clockwise
            [0, -1] => Dir8::N,
            [1, -1] => Dir8::NE,
            [1, 0] => Dir8::E,
            [1, 1] => Dir8::SE,
            [0, 1] => Dir8::S,
            [-1, 1] => Dir8::SW,
            [-1, 0] => Dir8::W,
            [-1, -1] => Dir8::NW,
            _ => unreachable!("unable to create Dir8 from virtual input"),
        })
    }
}
