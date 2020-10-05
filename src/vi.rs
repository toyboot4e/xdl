//! Virtual input, bundles of input states
//!
//! # Coordinate system
//!
//! X axis goes from left to right. Y axis goes from up to down. If not.. sorry!
//!
//! # Priority
//!
//! Latest inputs always come as current state.
//!
//! # About
//!
//! Virtual input is good for typical input abstraction. For example, your "select key" may be any
//! of enter, space, a gamepad button or even left click. Then virtual input is perfect for bundling
//! them.
//!
//! However, they are not generic enough. For example, you might want to handle left click in a
//! different way from enter key. Then you have to build your custom input system like UI commands,
//! maybe using virtual input.
//!
//! # Example
//!
//! Setting up axis input:
//!
//! ```rust
//! use std::time::Duration;
//!
//! use xdl::{
//!     vi::{AxisDirButton, InputBundle, KeyRepeat},
//!     Key,
//! };
//!
//! let dir = AxisDirButton::new(
//!     KeyRepeat::Repeat {
//!         first: Duration::new(0, 16666666) * 8,
//!         multi: Duration::new(0, 16666666) * 6,
//!     },
//!     [
//!          // positive input in x axis:
//!          InputBundle {
//!              keys: vec![Key::D, Key::Right],
//!              mouse: vec![],
//!          },
//!          // negative input in x axis:
//!          InputBundle {
//!              keys: vec![Key::A, Key::Left],
//!              mouse: vec![],
//!          },
//!     ],
//!     [
//!          // negative input in y axis:
//!          InputBundle {
//!              keys: vec![Key::S, Key::Down],
//!              mouse: vec![],
//!          },
//!          // negative input in y axis:
//!          InputBundle {
//!               keys: vec![Key::W, Key::Up],
//!               mouse: vec![],
//!          },
//!     ],
//! );
//! ```
//!
//! Then call `update` when you update your game!

use std::time::Duration;

use crate::{
    axis::{Dir4, Dir8, Sign},
    Input, Key, MouseInput,
};

/// Key repeat settings
#[derive(Debug, Clone, Copy)]
pub enum KeyRepeat {
    Repeat { first: Duration, multi: Duration },
    None,
}

/// Constructors
impl KeyRepeat {
    pub fn repeat(first: Duration, multi: Duration) -> Self {
        KeyRepeat::Repeat { first, multi }
    }

    pub fn no_repeat() -> Self {
        KeyRepeat::None
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

#[derive(Debug, Clone)]
struct KeyRepeatState {
    repeat: KeyRepeat,
    /// Loops when it repeats
    accum_repeat: Duration,
    /// Does not loop
    accum_down: Duration,
    /// True until first repeat
    is_on_first_repeat: bool,
}

impl KeyRepeatState {
    pub fn new(repeat: KeyRepeat) -> Self {
        Self {
            repeat,
            accum_repeat: Duration::new(0, 0),
            accum_down: Duration::new(0, 0),
            is_on_first_repeat: false,
        }
    }
}

/// Lifecycle
impl KeyRepeatState {
    /// Returns if it's repeating or not
    fn update(&mut self, state: RawButtonState, delta: Duration) -> bool {
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
                let repeat_duration = match self.repeat {
                    KeyRepeat::None => return false,
                    KeyRepeat::Repeat { first, multi } => {
                        if self.is_on_first_repeat {
                            first
                        } else {
                            multi
                        }
                    }
                };

                self.accum_repeat += delta;
                self.accum_down += delta;

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

/// Set of any kind of inputs
#[derive(Debug, Clone)]
pub struct InputBundle {
    pub keys: Vec<Key>,
    pub mouse: Vec<MouseInput>,
}

impl InputBundle {
    fn state(&self, input: &Input) -> RawButtonState {
        let mut is_any_down = false;
        let mut is_any_released = false;

        for key in self.keys.iter().map(|k| k.clone()) {
            if input.kbd.is_key_pressed(key) {
                return RawButtonState::Pressed;
            }
            is_any_down |= input.kbd.is_key_down(key);
            is_any_released |= input.kbd.is_key_released(key);
        }

        for m in self.mouse.iter().map(|m| m.clone()) {
            if input.mouse.is_pressed(m) {
                return RawButtonState::Pressed;
            }
            is_any_down |= input.mouse.is_down(m);
            is_any_released |= input.mouse.is_released(m);
        }

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
pub enum StrictButtonState {
    Down,
    Up,
    Pressed,
    Repeating,
    Released,
}

/// Input bundle with repeat state
#[derive(Debug, Clone)]
pub struct Button {
    bundle: InputBundle,
    state: StrictButtonState,
    repeat: KeyRepeatState,
}

impl Button {
    pub fn new(bundle: InputBundle, repeat: KeyRepeat) -> Self {
        Self {
            bundle,
            state: StrictButtonState::Up,
            repeat: KeyRepeatState::new(repeat),
        }
    }
}

/// Lifecycle
impl Button {
    pub fn update(&mut self, input: &Input, delta: Duration) {
        let state = self.bundle.state(input);
        let is_repeating = self.repeat.update(state, delta);
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
// Semantic buttons

/// Neg | Pos | Neutral
#[derive(Debug, Clone)]
pub struct AxisButton {
    /// Positive input
    pub pos: Button,
    /// Negative input
    pub neg: Button,
}

/// Lifecycle
impl AxisButton {
    pub fn update(&mut self, input: &Input, delta: Duration) {
        self.pos.update(input, delta);
        self.neg.update(input, delta);
    }
}

impl AxisButton {
    /// Down | Pressed | Repeating
    fn is_down(state: StrictButtonState) -> bool {
        matches!(
            state,
            StrictButtonState::Down | StrictButtonState::Pressed | StrictButtonState::Repeating
        )
    }

    /// Pressed | Repeating
    fn is_pressed(state: StrictButtonState) -> bool {
        matches!(
            state,
            StrictButtonState::Pressed | StrictButtonState::Repeating
        )
    }

    /// Selects down sign pressed lately
    pub fn sign_down(&self) -> Sign {
        let p_down = Self::is_down(self.pos.state);
        let n_down = Self::is_down(self.neg.state);

        match [p_down, n_down] {
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
        let p_down = Self::is_down(self.pos.state);
        let n_down = Self::is_down(self.neg.state);
        match [p_down, n_down] {
            [true, true] => {
                if self.pos.repeat.accum_down <= self.neg.repeat.accum_down {
                    if Self::is_pressed(self.pos.state) {
                        Sign::Pos
                    } else {
                        Sign::Neutral
                    }
                } else {
                    if Self::is_pressed(self.neg.state) {
                        Sign::Neg
                    } else {
                        Sign::Neutral
                    }
                }
            }
            [true, false] => {
                if Self::is_pressed(self.pos.state) {
                    Sign::Pos
                } else {
                    Sign::Neutral
                }
            }
            [false, true] => {
                if Self::is_pressed(self.neg.state) {
                    Sign::Neg
                } else {
                    Sign::Neutral
                }
            }
            [false, false] => Sign::Neutral,
        }
    }

    pub fn accum_down(&self) -> Duration {
        // select sign down lately
        std::cmp::min(self.pos.repeat.accum_down, self.neg.repeat.accum_down)
    }
}

/// [x, y] axes translated as direction
///
/// [x, y] components are "mixed" to make direction. For example, [1, 1] is interpreted as
/// south-east.
#[derive(Debug, Clone)]
pub struct AxisDirButton {
    x: AxisButton,
    y: AxisButton,
}

impl AxisDirButton {
    /// Creates axis from [positive, negative] input bundle in (x, y) axis
    ///
    /// Makes sure that the key repeat configuration is shared among buttons (while the states are
    /// not shared).
    pub fn new(repeat: KeyRepeat, xs: [InputBundle; 2], ys: [InputBundle; 2]) -> Self {
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
    pub fn update(&mut self, input: &Input, delta: Duration) {
        self.x.update(input, delta);
        self.y.update(input, delta);
    }
}

impl AxisDirButton {
    /// Creates direction mixing axis inputs
    pub fn to_dir4(&self) -> Option<Dir4> {
        // mix down inputs (not pressed inputs)
        let x = self.x.sign_down().to_i8();
        let y = self.y.sign_down().to_i8();

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

    pub fn to_dir8(&self) -> Option<Dir8> {
        let x = self.x.sign_pressed().to_i8();
        let y = self.y.sign_pressed().to_i8();

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
            _ => unreachable!(),
        })
    }
}

// TODO: add numpad-like direction where [x, y] components are not mixed
