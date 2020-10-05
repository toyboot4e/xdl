//! Mouse state
//!
//! * `x1`: first extended mouse button

use sdl2::event::Event;

use crate::utils::Double;

/// All of the mouse states
#[derive(Debug, Clone)]
pub struct Mouse {
    window: *mut sdl2::sys::SDL_Window,
    /// Mouse position and buttons (current/previous)
    mouses: Double<MouseSnapshot>,
    /// Mouse wheels (current/previous)
    wheels: Double<i32>,
}

/// XDL mouse input code
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum MouseInput {
    Left = sdl2::sys::SDL_BUTTON_LEFT,
    Right = sdl2::sys::SDL_BUTTON_RIGHT,
    Mid = sdl2::sys::SDL_BUTTON_MIDDLE,
    /// First external button
    X1 = sdl2::sys::SDL_BUTTON_X1,
    /// Second external button
    X2 = sdl2::sys::SDL_BUTTON_X2,
}

impl Mouse {
    pub fn new(window: *mut sdl2::sys::SDL_Window) -> Self {
        Self {
            window,
            mouses: Double::default(),
            wheels: Double::default(),
        }
    }
}

impl Mouse {
    pub fn event(&mut self, ev: &Event) {
        match ev {
            Event::MouseWheel { y, .. } => {
                // 120 units per notch
                self.wheels.b += y * 120;
            }
            _ => {}
        }
    }

    pub fn update(&mut self) {
        let mut x = 0;
        let mut y = 0;

        let support_global_mouse_mode = true;

        let flags = unsafe {
            if sdl2::sys::SDL_GetRelativeMouseMode() == sdl2::sys::SDL_bool::SDL_TRUE {
                sdl2::sys::SDL_GetRelativeMouseState(&mut x, &mut y)
            } else if support_global_mouse_mode {
                let flags = sdl2::sys::SDL_GetGlobalMouseState(&mut x, &mut y);
                let (mut wx, mut wy) = (0, 0);
                sdl2::sys::SDL_GetWindowPosition(self.window, &mut wx, &mut wy);
                x -= wx;
                y -= wy;
                flags
            } else {
                // inaccurate
                sdl2::sys::SDL_GetMouseState(&mut x, &mut y)
            }
        };

        // TODO: consider resolution scale
        // x = (i32) ((f32) x * INTERNAL_BackBufferWidth / INTERNAL_WindowWidth);
        // y = (i32) ((f32) y * INTERNAL_BackBufferHeight / INTERNAL_WindowHeight);

        let snapshot = MouseSnapshot { x, y, flags };
        self.mouses.b = snapshot;
    }

    pub fn on_end_frame(&mut self) {
        self.mouses.a = self.mouses.b.clone();
        self.wheels.a = self.wheels.b.clone();
    }
}

impl Mouse {
    pub fn x(&self) -> i32 {
        self.mouses.b.x()
    }

    pub fn y(&self) -> i32 {
        self.mouses.b.y()
    }

    pub fn pos(&self) -> [i32; 2] {
        [self.mouses.b.x(), self.mouses.b.y()]
    }

    pub fn pos_delta(&self) -> [i32; 2] {
        [
            self.mouses.b.x() - self.mouses.a.x(),
            self.mouses.b.y() - self.mouses.a.y(),
        ]
    }

    // TODO: scaled mouse position, multiplying resolution scale
}

/// Down
impl Mouse {
    pub fn is_left_down(&self) -> bool {
        self.mouses.b.is_left_down()
    }

    pub fn is_mid_down(&self) -> bool {
        self.mouses.b.is_mid_down()
    }

    pub fn is_right_down(&self) -> bool {
        self.mouses.b.is_right_down()
    }

    pub fn is_x1_down(&self) -> bool {
        self.mouses.b.is_x1_down()
    }

    pub fn is_x2_down(&self) -> bool {
        self.mouses.b.is_x2_down()
    }
}

/// Up
impl Mouse {
    pub fn is_left_up(&self) -> bool {
        self.mouses.b.is_left_up()
    }

    pub fn is_mid_up(&self) -> bool {
        self.mouses.b.is_mid_up()
    }

    pub fn is_right_up(&self) -> bool {
        self.mouses.b.is_right_up()
    }

    pub fn is_x1_up(&self) -> bool {
        self.mouses.b.is_x1_up()
    }

    pub fn is_x2_up(&self) -> bool {
        self.mouses.b.is_x2_up()
    }
}

/// Pressed/released
impl Mouse {
    pub fn is_left_pressed(&self) -> bool {
        self.mouses.b.is_left_down() && !self.mouses.a.is_left_down()
    }

    pub fn is_left_released(&self) -> bool {
        !self.mouses.b.is_left_down() && self.mouses.a.is_left_down()
    }

    pub fn is_right_pressed(&self) -> bool {
        self.mouses.b.is_right_down() && !self.mouses.a.is_right_down()
    }

    pub fn is_right_released(&self) -> bool {
        !self.mouses.b.is_right_down() && self.mouses.a.is_right_down()
    }

    pub fn is_mid_pressed(&self) -> bool {
        self.mouses.b.is_mid_down() && !self.mouses.a.is_mid_down()
    }

    pub fn is_mid_released(&self) -> bool {
        !self.mouses.b.is_mid_down() && self.mouses.a.is_mid_down()
    }

    pub fn is_x1_pressed(&self) -> bool {
        self.mouses.b.is_x1_down() && !self.mouses.a.is_x1_down()
    }

    pub fn is_x1_released(&self) -> bool {
        !self.mouses.b.is_x1_down() && self.mouses.a.is_x1_down()
    }

    pub fn is_x2_pressed(&self) -> bool {
        self.mouses.b.is_x2_down() && !self.mouses.a.is_x2_down()
    }

    pub fn is_x2_released(&self) -> bool {
        !self.mouses.b.is_x2_down() && self.mouses.a.is_x2_down()
    }
}

/// [`MouseInput`]
impl Mouse {
    pub fn is_down(&self, input: MouseInput) -> bool {
        self.mouses.b.is_down(input)
    }

    pub fn is_up(&self, input: MouseInput) -> bool {
        self.mouses.b.is_up(input)
    }

    pub fn is_pressed(&self, input: MouseInput) -> bool {
        self.mouses.b.is_down(input) && !self.mouses.a.is_down(input)
    }

    pub fn is_released(&self, input: MouseInput) -> bool {
        !self.mouses.b.is_down(input) && self.mouses.a.is_down(input)
    }
}

/// Multiple inputs
impl Mouse {
    pub fn is_any_down<'a>(&self, inputs: impl IntoIterator<Item = &'a MouseInput>) -> bool {
        inputs.into_iter().any(|input| self.is_down(*input))
    }

    pub fn is_any_up<'a>(&self, inputs: impl IntoIterator<Item = &'a MouseInput>) -> bool {
        inputs.into_iter().any(|input| self.is_up(*input))
    }

    pub fn is_any_pressed<'a>(&self, inputs: impl IntoIterator<Item = &'a MouseInput>) -> bool {
        inputs.into_iter().any(|input| self.is_down(*input))
    }

    pub fn is_any_released<'a>(&self, inputs: impl IntoIterator<Item = &'a MouseInput>) -> bool {
        inputs.into_iter().any(|input| self.is_down(*input))
    }
}

/// Represents a mouse state with cursor position and button press information.
///
/// Basically `sdl2::mouse::MouseState` but with backbuffer size and mouse mode.
///
/// * Relative mouse position is relative to the window
/// * Global mouse position is relative to the top-left corner of the desktop
#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct MouseSnapshot {
    pub x: i32,
    pub y: i32,
    flags: u32,
}

impl MouseSnapshot {
    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }
}

/// Down
impl MouseSnapshot {
    fn mask(button: u32) -> u32 {
        1 << (button - 1)
    }

    pub fn is_left_down(&self) -> bool {
        (self.flags & Self::mask(sdl2::sys::SDL_BUTTON_LEFT)) != 0
    }

    pub fn is_mid_down(&self) -> bool {
        (self.flags & Self::mask(sdl2::sys::SDL_BUTTON_MIDDLE)) != 0
    }

    pub fn is_right_down(&self) -> bool {
        (self.flags & Self::mask(sdl2::sys::SDL_BUTTON_RIGHT)) != 0
    }

    pub fn is_x1_down(&self) -> bool {
        (self.flags & Self::mask(sdl2::sys::SDL_BUTTON_X1)) != 0
    }

    pub fn is_x2_down(&self) -> bool {
        (self.flags & Self::mask(sdl2::sys::SDL_BUTTON_X2)) != 0
    }
}

/// Up
impl MouseSnapshot {
    pub fn is_left_up(&self) -> bool {
        !self.is_left_down()
    }

    pub fn is_mid_up(&self) -> bool {
        !self.is_mid_down()
    }

    pub fn is_right_up(&self) -> bool {
        !self.is_right_down()
    }

    pub fn is_x1_up(&self) -> bool {
        !self.is_x1_down()
    }

    pub fn is_x2_up(&self) -> bool {
        !self.is_x2_down()
    }
}

/// [`MouseInput`]
impl MouseSnapshot {
    pub fn is_down(&self, input: MouseInput) -> bool {
        (self.flags & Self::mask(input as u32)) != 0
    }

    pub fn is_up(&self, input: MouseInput) -> bool {
        !(self.is_down(input))
    }

    pub fn is_any_down<'a>(&self, inputs: impl IntoIterator<Item = &'a MouseInput>) -> bool {
        inputs.into_iter().any(|input| self.is_down(*input))
    }

    pub fn is_any_up<'a>(&self, inputs: impl IntoIterator<Item = &'a MouseInput>) -> bool {
        inputs.into_iter().any(|input| self.is_up(*input))
    }
}
