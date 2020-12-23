pub mod keyboard;
// pub mod mouse;

use self::keyboard::Keyboard;

/// All of the input states
#[derive(Debug, Clone)]
pub struct Input {
    pub kbd: Keyboard,
    // pub mouse: Mouse,
}

#[cfg(feature = "use-sdl2")]
impl Input {
    pub fn new(win: *mut sdl2::sys::SDL_Window) -> Self {
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

#[cfg(feature = "use-rokol")]
impl Input {
    pub fn new() -> Self {
        Self {
            kbd: Keyboard::default(),
        }
    }

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

    /// Resets all states (useful when e.g. losing window focus and can't receive key released event)
    pub fn clear(&mut self) {
        self.kbd.clear();
        // self.mouse.clear();
    }
}
