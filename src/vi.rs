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

X axis goes from left to right. Y axis goes from up to down. If not.. sorry!

# Priority

Latest inputs always come as current state.

# TODOs

* handle modifier keys

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
#[cfg_attr(feature = "use-serde", derive(Serialize, Deserialize))]
pub enum KeyRepeatConfig {
    Repeat { first: Duration, multi: Duration },
    None,
}

/// Constructors
impl KeyRepeatConfig {
    pub fn repeat(first: Duration, multi: Duration) -> Self {
        KeyRepeatConfig::Repeat { first, multi }
    }

    pub fn no_repeat() -> Self {
        KeyRepeatConfig::None
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
#[cfg_attr(feature = "use-serde", derive(Serialize, Deserialize))]
struct KeyRepeatState {
    /// Key repeat configuration
    repeat: KeyRepeatConfig,
    /// Loops when it repeats
    accum_repeat: Duration,
    /// Does not loop
    #[cfg_attr(feature = "use-serde", serde(skip))]
    accum_down: Duration,
    /// True until first repeat
    #[cfg_attr(feature = "use-serde", serde(skip))]
    is_on_first_repeat: bool,
}

impl KeyRepeatState {
    pub fn new(repeat: KeyRepeatConfig) -> Self {
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
                let repeat_duration = match self.repeat {
                    KeyRepeatConfig::None => return false,
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "use-serde", derive(Serialize, Deserialize), serde(untagged))]
pub enum KeyEntry {
    Key1([Key; 1]),
    /// control+f
    Key2([Key; 2]),
    /// control+shift+f
    Key3([Key; 3]),
    /// control+shift+cmd+f
    Key4([Key; 4]),
}

impl KeyEntry {
    pub fn as_slice(&self) -> &[Key] {
        match self {
            Self::Key1(ks) => ks,
            Self::Key2(ks) => ks,
            Self::Key3(ks) => ks,
            Self::Key4(ks) => ks,
        }
    }
}

/// Set of any kind of inputs
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "use-serde", derive(Serialize, Deserialize))]
pub struct InputBundle {
    pub keys: Vec<KeyEntry>,
    // pub mouse: Vec<MouseInput>,
}

impl InputBundle {
    fn state(&self, input: &Input) -> RawButtonState {
        let mut is_any_down = false;
        let mut is_any_released = false;

        for entry in self.keys.iter().cloned() {
            let mut is_pressed = true;
            let mut is_down = true;
            let mut is_down_prev = true;
            for key in entry.as_slice().iter().cloned() {
                is_pressed &= input.kbd.is_key_pressed(key);
                is_down &= input.kbd.is_key_down(key);
                is_down_prev |= input.kbd.snaps.b.is_down(key);
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
#[cfg_attr(feature = "use-serde", derive(Serialize, Deserialize))]
pub enum StrictButtonState {
    Down,
    Up,
    Pressed,
    Repeating,
    Released,
}

fn default_strict_button_state() -> StrictButtonState {
    StrictButtonState::Up
}

/// Input bundle with repeat state
#[derive(Debug, Clone)]
// #[cfg_attr(feature = "use-serde", derive(Serialize, Deserialize))]
#[derive(Serialize, Deserialize)]
pub struct Button {
    pub bundle: InputBundle,
    #[serde(skip, default = "default_strict_button_state")]
    pub state: StrictButtonState,
    repeat: KeyRepeatState,
}

impl Button {
    pub fn new(bundle: InputBundle, repeat: KeyRepeatConfig) -> Self {
        Self {
            bundle,
            state: StrictButtonState::Up,
            repeat: KeyRepeatState::new(repeat),
        }
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
        let state = self.bundle.state(input);

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
#[derive(Debug, Clone)]
#[cfg_attr(feature = "use-serde", derive(Serialize, Deserialize))]
pub struct AxisButton {
    /// Positive input
    pub pos: Button,
    /// Negative input
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
///
/// # Example
///
/// ```rust
/// use std::time::Duration;
///
/// use xdl::{
///     vi::{AxisDirButton, InputBundle, KeyRepeat},
///     Key,
/// };
///
/// let dir = AxisDirButton::new(
///     KeyRepeat::Repeat {
///         first: Duration::from_nanos(1_000_000_000 / 60 * 8),
///         multi: Duration::from_nanos(1_000_000_000 / 60 * 6),
///     },
///     [
///          // positive input in x axis:
///          InputBundle {
///              keys: vec![Key::D, Key::Right],
///              ..Default::default()
///          },
///          // negative input in x axis:
///          InputBundle {
///              keys: vec![Key::A, Key::Left],
///              ..Default::default()
///          },
///     ],
///     [
///          // positive input in y axis:
///          InputBundle {
///              keys: vec![Key::S, Key::Down],
///              ..Default::default()
///          },
///          // negative input in y axis:
///          InputBundle {
///              keys: vec![Key::W, Key::Up],
///              ..Default::default()
///          },
///     ],
/// );
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "use-serde", derive(Serialize, Deserialize))]
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
