//! Virtual input, bundles of input states
//!
//! # Coordinate system
//!
//! X axis goes from left to right. Y axis goes from up to down. If not.. sorry!
//!
//! # Priority
//!
//! Lazy input always comes as current state.
//!
//! # Usage
//!
//! It's good for typical input abstraction. For example, your "select key" may be any of enter,
//! space, a gamepad button or even left click. Then virtual input is perfect for bundling them.
//!
//! However, they are not generic enough. For example, you might want to handle left click in a
//! different way from enter key. Then you have to build your custom input system like UI commands,
//! maybe on top of virtual input.
//!
//! # Example
//!
//! ```rust
//! use xdl::Key;
//! use xdl::vi::{AxisButton, Button, DirButton, InputBundle, KeyRepeat};
//!
//! let dir = DirButton {
//!     x: AxisButton {
//!         pos: Button::new(
//!             InputBundle {
//!                 keys: vec![Key::D, Key::Right],
//!                 mouse: vec![],
//!             },
//!             KeyRepeat::None,
//!         ),
//!         neg: Button::new(
//!             InputBundle {
//!                 keys: vec![Key::A, Key::Left],
//!                 mouse: vec![],
//!             },
//!             KeyRepeat::None,
//!         ),
//!     },
//!     y: AxisButton {
//!         pos: Button::new(
//!             InputBundle {
//!                 keys: vec![Key::S, Key::Down],
//!                 mouse: vec![],
//!             },
//!             KeyRepeat::None,
//!         ),
//!         neg: Button::new(
//!             InputBundle {
//!                 keys: vec![Key::W, Key::Up],
//!                 mouse: vec![],
//!             },
//!             KeyRepeat::None,
//!         ),
//!     },
//! };
//! ```
//!

use std::time::Duration;

use crate::{
    axis::{Dir4, Dir8, Sign},
    Input, Key, MouseInput,
};

// --------------------------------------------------------------------------------
// Key repeat, button state and input bundle

/// Key repeat settings
#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
struct KeyRepeatState {
    repeat: KeyRepeat,
    /// Loops when it repeats
    accum_repeat: Duration,
    /// Does not loop
    accum_down: Duration,
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
    pub fn update(&mut self, state: StrictButtonState, delta: Duration) -> bool {
        match state {
            StrictButtonState::Down => {
                let target = match self.repeat {
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
                self.is_on_first_repeat = false;

                let mut repeat = false;

                // basically it's just an if branch but in case too long time passed
                while self.accum_repeat > target {
                    repeat = true;
                    self.accum_repeat -= target;
                }

                repeat
            }
            StrictButtonState::Up | StrictButtonState::Released => {
                self.accum_repeat = Duration::new(0, 0);
                self.accum_down = Duration::new(0, 0);
                self.is_on_first_repeat = false;
                false
            }
            StrictButtonState::Pressed => {
                self.accum_repeat = Duration::new(0, 0);
                self.accum_down = Duration::new(0, 0);
                self.is_on_first_repeat = true;
                false
            }
        }
    }
}

/// Down | Up | Pressed | Released
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StrictButtonState {
    Down,
    Up,
    Pressed,
    Released,
}

/// Set of any kind of inputs
#[derive(Debug, Clone)]
pub struct InputBundle {
    pub keys: Vec<Key>,
    pub mouse: Vec<MouseInput>,
}

impl InputBundle {
    fn state(&self, input: &Input) -> StrictButtonState {
        let mut is_any_down = false;
        let mut is_any_released = false;

        for key in self.keys.iter().map(|k| k.clone()) {
            if input.kbd.is_key_pressed(key) {
                return StrictButtonState::Pressed;
            }
            is_any_down |= input.kbd.is_key_down(key);
            is_any_released |= input.kbd.is_key_released(key);
        }

        for m in self.mouse.iter().map(|m| m.clone()) {
            if input.mouse.is_pressed(m) {
                return StrictButtonState::Pressed;
            }
            is_any_down |= input.mouse.is_down(m);
            is_any_released |= input.mouse.is_released(m);
        }

        if is_any_down {
            StrictButtonState::Down
        } else {
            if is_any_released {
                StrictButtonState::Released
            } else {
                StrictButtonState::Up
            }
        }
    }
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
            StrictButtonState::Pressed
        } else {
            state
        };
    }
}

// --------------------------------------------------------------------------------
// Semantic input

/// Down | Up | Pressed | Released with [`Sign`]
#[derive(Debug, Clone)]
pub enum StrictAxisState {
    Down(Sign),
    Up,
    Pressed(Sign),
    Released(Sign),
}

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
    pub fn sign_down(&self) -> Sign {
        match [self.pos.state, self.neg.state] {
            [StrictButtonState::Down, StrictButtonState::Down] => {
                // select axis down earlier
                if self.pos.repeat.accum_down <= self.neg.repeat.accum_down {
                    Sign::Pos
                } else {
                    Sign::Neg
                }
            }
            [StrictButtonState::Down, _] | [StrictButtonState::Pressed, _] => Sign::Pos,
            [_, StrictButtonState::Down] | [_, StrictButtonState::Pressed] => Sign::Neg,
            _ => Sign::Neutral,
        }
    }

    pub fn sign_pressed(&self) -> Sign {
        match [self.pos.state, self.neg.state] {
            [StrictButtonState::Pressed, _] => Sign::Pos,
            [_, StrictButtonState::Pressed] => Sign::Neg,
            _ => Sign::Neutral,
        }
    }

    pub fn state(&self) -> StrictAxisState {
        match [self.pos.state, self.neg.state] {
            [StrictButtonState::Pressed, _] => StrictAxisState::Pressed(Sign::Pos),
            [_, StrictButtonState::Pressed] => StrictAxisState::Pressed(Sign::Neg),
            [StrictButtonState::Down, StrictButtonState::Down] => {
                // select axis down earlier
                if self.pos.repeat.accum_down <= self.neg.repeat.accum_down {
                    StrictAxisState::Down(Sign::Pos)
                } else {
                    StrictAxisState::Down(Sign::Neg)
                }
            }
            [StrictButtonState::Down, _] => StrictAxisState::Down(Sign::Pos),
            [_, StrictButtonState::Down] => StrictAxisState::Down(Sign::Neg),
            [StrictButtonState::Released, _] => StrictAxisState::Released(Sign::Pos),
            [_, StrictButtonState::Released] => StrictAxisState::Released(Sign::Neg),
            _ => StrictAxisState::Up,
        }
    }

    pub fn accum_down(&self) -> Duration {
        // select axis down earlier
        std::cmp::min(self.pos.repeat.accum_down, self.neg.repeat.accum_down)
    }
}

/// Direction button for orthogonal grid map
#[derive(Debug, Clone)]
pub struct DirButton {
    pub x: AxisButton,
    pub y: AxisButton,
}

/// Lifecycle
impl DirButton {
    pub fn update(&mut self, input: &Input, delta: Duration) {
        self.x.update(input, delta);
        self.y.update(input, delta);
    }
}

impl DirButton {
    pub fn to_dir4(&self) -> Option<Dir4> {
        let x = self.x.sign_pressed().to_i32();
        let y = self.y.sign_pressed().to_i32();

        Some(match [x, y] {
            [0, 0] => return None,
            // clockwise
            [0, -1] => Dir4::N,
            [1, 0] => Dir4::E,
            [0, 1] => Dir4::S,
            [-1, 0] => Dir4::W,
            [_, _] => {
                // select axis down earlier
                if self.x.accum_down() <= self.y.accum_down() {
                    match x {
                        1 => Dir4::E,
                        -1 => Dir4::W,
                        _ => unreachable!(),
                    }
                } else {
                    match y {
                        -1 => Dir4::N,
                        1 => Dir4::S,
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
